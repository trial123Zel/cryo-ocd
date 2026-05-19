use std::env;

use crate::args::Args;
use alloy::{
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::client::{ClientBuilder, RpcClient},
    transports::{layers::RetryBackoffLayer, Authorization},
};
use alloy_rpc_types_engine::{Claims, JwtSecret};
use alloy_transport_http::{AuthLayer, Http, HyperClient};
use cryo_freeze::{ParseError, Source, SourceLabels};
use governor::{Quota, RateLimiter};
use polars::prelude::*;
use std::num::NonZeroU32;

pub(crate) async fn parse_source(args: &Args) -> Result<Source, ParseError> {
    // parse network info
    let rpc_url = parse_rpc_url(args)?;
    let retry_layer = RetryBackoffLayer::new(
        args.max_retries,
        args.initial_backoff,
        args.compute_units_per_second,
    );
    let client = build_client(&rpc_url, retry_layer, args.jwt_secret.as_deref()).await?;
    let provider = ProviderBuilder::default().connect_client(client).erased();
    let chain_id = provider.get_chain_id().await.map_err(ParseError::ProviderError)?;
    let rate_limiter = match args.requests_per_second {
        Some(rate_limit) => match (NonZeroU32::new(1), NonZeroU32::new(rate_limit)) {
            (Some(one), Some(value)) => {
                let quota = Quota::per_second(value).allow_burst(one);
                Some(RateLimiter::direct(quota))
            }
            _ => None,
        },
        None => None,
    };

    // process concurrency info
    let max_concurrent_requests = args.max_concurrent_requests.unwrap_or(100);
    let max_concurrent_chunks = match args.max_concurrent_chunks {
        Some(0) => None,
        Some(max) => Some(max),
        None => Some(4),
    };

    let semaphore = tokio::sync::Semaphore::new(max_concurrent_requests as usize);
    let semaphore = Arc::new(Some(semaphore));

    let output = Source {
        chain_id,
        inner_request_size: args.inner_request_size,
        max_concurrent_chunks,
        semaphore,
        rate_limiter: rate_limiter.into(),
        rpc_url,
        provider,
        labels: SourceLabels {
            max_concurrent_requests: args.max_concurrent_requests,
            max_requests_per_second: args.requests_per_second.map(|x| x as u64),
            max_retries: Some(args.max_retries),
            initial_backoff: Some(args.initial_backoff),
        },
    };

    Ok(output)
}

/// Build the RPC client for `rpc_url`, wiring in JWT authentication when a
/// `--jwt-secret` is supplied.
///
/// The secret is consumed transiently: it is parsed into a [`JwtSecret`], used
/// to construct the transport, and dropped. It is never stored on [`Source`]
/// (which derives `Debug`), logged, or written to the run report.
///
/// With no `--jwt-secret`, this is the original transport-agnostic
/// `ClientBuilder::connect` path, unchanged.
async fn build_client(
    rpc_url: &str,
    retry_layer: RetryBackoffLayer,
    jwt_secret: Option<&str>,
) -> Result<RpcClient, ParseError> {
    let Some(jwt_secret) = jwt_secret else {
        return ClientBuilder::default()
            .layer(retry_layer)
            .connect(rpc_url)
            .await
            .map_err(ParseError::ProviderError)
    };

    let secret = parse_jwt_secret(jwt_secret)?;

    if rpc_url.ends_with(".ipc") {
        // IPC is a local socket with no HTTP layer to carry an `Authorization`
        // header; access is controlled by filesystem permissions. Ignore the
        // secret and connect normally.
        eprintln!("warning: --jwt-secret is ignored for IPC connections");
        ClientBuilder::default()
            .layer(retry_layer)
            .connect(rpc_url)
            .await
            .map_err(ParseError::ProviderError)
    } else if rpc_url.starts_with("ws") {
        // WebSocket: a freshly-minted bearer token authenticates the handshake.
        let token = secret
            .encode(&Claims::with_current_timestamp())
            .map_err(|_| ParseError::ParseError("could not sign JWT".to_string()))?;
        let ws = WsConnect::new(rpc_url).with_auth(Authorization::bearer(token));
        ClientBuilder::default().layer(retry_layer).ws(ws).await.map_err(ParseError::ProviderError)
    } else {
        // HTTP(S): `AuthLayer` mints and auto-refreshes the bearer token,
        // adding it as an `Authorization` header on every request. The hyper
        // transport links native-tls, so both http:// and https:// work.
        let url: url::Url = rpc_url.parse().map_err(ParseError::ParseUrlError)?;
        let hyper_client = HyperClient::new().layer(AuthLayer::new(secret));
        let transport = Http::with_client(hyper_client, url);
        let is_local = transport.guess_local();
        Ok(ClientBuilder::default().layer(retry_layer).transport(transport, is_local))
    }
}

/// Parse a `--jwt-secret` value: either a path to a file containing the
/// secret, or a 64-character hex string.
///
/// Errors deliberately do not echo the value, which may be the secret itself.
fn parse_jwt_secret(value: &str) -> Result<JwtSecret, ParseError> {
    let path = std::path::Path::new(value);
    if path.is_file() {
        JwtSecret::from_file(path)
            .map_err(|e| ParseError::ParseError(format!("invalid JWT secret file: {e}")))
    } else {
        JwtSecret::from_hex(value).map_err(|_| {
            ParseError::ParseError(
                "invalid --jwt-secret (expected a 64-char hex string or a path to a secret file)"
                    .to_string(),
            )
        })
    }
}

pub(crate) fn parse_rpc_url(args: &Args) -> Result<String, ParseError> {
    // get MESC url
    let mesc_url = if mesc::is_mesc_enabled() {
        let endpoint = match &args.rpc {
            Some(url) => mesc::get_endpoint_by_query(url, Some("cryo")),
            None => mesc::get_default_endpoint(Some("cryo")),
        };
        match endpoint {
            Ok(endpoint) => endpoint.map(|endpoint| endpoint.url),
            Err(e) => {
                eprintln!("Could not load MESC data: {}", e);
                None
            }
        }
    } else {
        None
    };

    // use ETH_RPC_URL if no MESC url found
    let url = if let Some(url) = mesc_url {
        url
    } else if let Some(url) = &args.rpc {
        url.clone()
    } else if let Ok(url) = env::var("ETH_RPC_URL") {
        url
    } else {
        let message = "must provide --rpc or setup MESC or set ETH_RPC_URL";
        return Err(ParseError::ParseError(message.to_string()))
    };

    // prepend http or https if need be
    if !url.starts_with("http") & !url.starts_with("ws") & !url.ends_with(".ipc") {
        Ok("http://".to_string() + url.as_str())
    } else {
        Ok(url)
    }
}
