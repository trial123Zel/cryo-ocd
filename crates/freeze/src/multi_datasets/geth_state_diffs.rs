use crate::*;
use alloy::{
    primitives::{Address, Bytes, B256, U256},
    rpc::types::trace::geth::{AccountState, DiffMode},
};
use polars::prelude::*;
use std::collections::{BTreeMap, HashMap, HashSet};

/// state diffs from geth debug traces
pub struct GethStateDiffs(
    pub Option<GethBalanceDiffs>,
    pub Option<GethCodeDiffs>,
    pub Option<GethNonceDiffs>,
    pub Option<GethStorageDiffs>,
);

impl Default for GethStateDiffs {
    fn default() -> GethStateDiffs {
        GethStateDiffs(
            Some(GethBalanceDiffs::default()),
            Some(GethCodeDiffs::default()),
            Some(GethNonceDiffs::default()),
            Some(GethStorageDiffs::default()),
        )
    }
}

impl ToDataFrames for GethStateDiffs {
    fn create_dfs(
        self,
        schemas: &HashMap<Datatype, Table>,
        chain_id: u64,
    ) -> R<HashMap<Datatype, DataFrame>> {
        let GethStateDiffs(balance_diffs, code_diffs, nonce_diffs, storage_diffs) = self;
        let mut output = HashMap::new();
        if let Some(balance_diffs) = balance_diffs {
            output.extend(balance_diffs.create_dfs(schemas, chain_id)?);
        }
        if let Some(code_diffs) = code_diffs {
            output.extend(code_diffs.create_dfs(schemas, chain_id)?);
        }
        if let Some(nonce_diffs) = nonce_diffs {
            output.extend(nonce_diffs.create_dfs(schemas, chain_id)?);
        }
        if let Some(storage_diffs) = storage_diffs {
            output.extend(storage_diffs.create_dfs(schemas, chain_id)?);
        }
        Ok(output)
    }
}

type BlockTxsTraces = (Option<u32>, Vec<Option<Vec<u8>>>, Vec<DiffMode>);

#[async_trait::async_trait]
impl CollectByBlock for GethStateDiffs {
    type Response = BlockTxsTraces;

    async fn extract(request: Params, source: Arc<Source>, query: Arc<Query>) -> R<Self::Response> {
        let block_number = request.block_number()? as u32;
        let include_txs = query.schemas.values().any(|x| x.has_column("transaction_hash"));
        source.geth_debug_trace_block_diffs(block_number, include_txs).await
    }

    fn transform(response: Self::Response, columns: &mut Self, query: &Arc<Query>) -> R<()> {
        let GethStateDiffs(ref mut balances, ref mut codes, ref mut nonces, ref mut storages) =
            columns;
        process_geth_diffs(
            &response,
            balances.as_mut(),
            codes.as_mut(),
            nonces.as_mut(),
            storages.as_mut(),
            &query.schemas,
        )
    }
}

#[async_trait::async_trait]
impl CollectByTransaction for GethStateDiffs {
    type Response = BlockTxsTraces;

    async fn extract(request: Params, source: Arc<Source>, query: Arc<Query>) -> R<Self::Response> {
        let include_block_number = query.schemas.values().any(|x| x.has_column("transaction_hash"));
        source
            .geth_debug_trace_transaction_diffs(request.transaction_hash()?, include_block_number)
            .await
    }

    fn transform(response: Self::Response, columns: &mut Self, query: &Arc<Query>) -> R<()> {
        let GethStateDiffs(ref mut balances, ref mut codes, ref mut nonces, ref mut storages) =
            columns;
        process_geth_diffs(
            &response,
            balances.as_mut(),
            codes.as_mut(),
            nonces.as_mut(),
            storages.as_mut(),
            &query.schemas,
        )
    }
}

pub(crate) fn process_geth_diffs(
    response: &BlockTxsTraces,
    mut balances: Option<&mut GethBalanceDiffs>,
    mut codes: Option<&mut GethCodeDiffs>,
    mut nonces: Option<&mut GethNonceDiffs>,
    mut storages: Option<&mut GethStorageDiffs>,
    schemas: &Schemas,
) -> R<()> {
    let (block_number, txs, traces) = response;
    let balance_schema = schemas.get(&Datatype::GethBalanceDiffs);
    let code_schema = schemas.get(&Datatype::GethCodeDiffs);
    let nonce_schema = schemas.get(&Datatype::GethNonceDiffs);
    let storage_schema = schemas.get(&Datatype::GethStorageDiffs);

    for (tx_index, (trace, tx)) in traces.iter().zip(txs).enumerate() {
        let index = &(*block_number, tx_index as u32, tx.clone());
        let addresses: Vec<_> = trace
            .pre
            .keys()
            .chain(trace.post.keys())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        for address in addresses.into_iter() {
            let (pre, post) = resolve_pre_post(trace.pre.get(address), trace.post.get(address));
            if let (Some(balances), Some(schema)) = (balances.as_mut(), balance_schema) {
                add_balances(address, pre.balance, post.balance, balances, schema, index)?;
            }
            if let (Some(codes), Some(schema)) = (codes.as_mut(), code_schema) {
                add_codes(address, &pre.code, &post.code, codes, schema, index)?;
            }
            if let (Some(nonces), Some(schema)) = (nonces.as_mut(), nonce_schema) {
                add_nonces(address, pre.nonce, post.nonce, nonces, schema, index)?;
            }
            if let (Some(storages), Some(schema)) = (storages.as_mut(), storage_schema) {
                add_storages(address, &pre.storage, &post.storage, storages, schema, index)?;
            }
        }
    }
    Ok(())
}

fn add_balances(
    address: &Address,
    pre: Option<U256>,
    post: Option<U256>,
    columns: &mut GethBalanceDiffs,
    schema: &Table,
    index: &(Option<u32>, u32, Option<Vec<u8>>),
) -> R<()> {
    let (from_value, to_value) = parse_pre_post(pre, post, U256::ZERO);
    let (block_number, transaction_index, transaction_hash) = index;
    columns.n_rows += 1;
    store!(schema, columns, block_number, *block_number);
    store!(schema, columns, transaction_index, Some(*transaction_index as u64));
    store!(schema, columns, transaction_hash, transaction_hash.clone());
    store!(schema, columns, address, address.to_vec());
    store!(schema, columns, from_value, from_value);
    store!(schema, columns, to_value, to_value);
    Ok(())
}

fn add_codes(
    address: &Address,
    pre: &Option<Bytes>,
    post: &Option<Bytes>,
    columns: &mut GethCodeDiffs,
    schema: &Table,
    index: &(Option<u32>, u32, Option<Vec<u8>>),
) -> R<()> {
    let blank = Bytes::new();
    let (from_value, to_value) = match (pre, post) {
        (Some(pre), Some(post)) => (pre, post),
        (Some(pre), None) => (pre, &blank),
        (None, Some(post)) => (&blank, post),
        (None, None) => (&blank, &blank),
    };
    let (block_number, transaction_index, transaction_hash) = index;
    columns.n_rows += 1;
    store!(schema, columns, block_number, *block_number);
    store!(schema, columns, transaction_index, Some(*transaction_index as u64));
    store!(schema, columns, transaction_hash, transaction_hash.clone());
    store!(schema, columns, address, address.to_vec());
    store!(schema, columns, from_value, from_value.to_vec());
    store!(schema, columns, to_value, to_value.to_vec());
    Ok(())
}

fn add_nonces(
    address: &Address,
    pre: Option<u64>,
    post: Option<u64>,
    columns: &mut GethNonceDiffs,
    schema: &Table,
    index: &(Option<u32>, u32, Option<Vec<u8>>),
) -> R<()> {
    let (from_value, to_value) = parse_pre_post(pre, post, 0_u64);
    let (block_number, transaction_index, transaction_hash) = index;
    columns.n_rows += 1;
    store!(schema, columns, block_number, *block_number);
    store!(schema, columns, transaction_index, Some(*transaction_index as u64));
    store!(schema, columns, transaction_hash, transaction_hash.clone());
    store!(schema, columns, address, address.to_vec());
    store!(schema, columns, from_value, from_value);
    store!(schema, columns, to_value, to_value);
    Ok(())
}

fn add_storages(
    address: &Address,
    pre: &BTreeMap<B256, B256>,
    post: &BTreeMap<B256, B256>,
    columns: &mut GethStorageDiffs,
    schema: &Table,
    index: &(Option<u32>, u32, Option<Vec<u8>>),
) -> R<()> {
    let (block_number, transaction_index, transaction_hash) = index;
    let slots: Vec<_> = pre
        .clone()
        .into_keys()
        .chain(post.clone().into_keys())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    let blank = B256::ZERO;
    for slot in slots.into_iter() {
        let (from, to) = match (pre.get(&slot), post.get(&slot)) {
            (Some(pre), Some(post)) => (pre, post),
            (Some(pre), None) => (pre, &blank),
            (None, Some(post)) => (&blank, post),
            (None, None) => (&blank, &blank),
        };
        columns.n_rows += 1;
        store!(schema, columns, block_number, *block_number);
        store!(schema, columns, transaction_index, Some(*transaction_index as u64));
        store!(schema, columns, transaction_hash, transaction_hash.clone());
        store!(schema, columns, address, address.to_vec());
        store!(schema, columns, slot, slot.to_vec());
        store!(schema, columns, from_value, from.to_vec());
        store!(schema, columns, to_value, to.to_vec());
    }
    Ok(())
}

fn parse_pre_post<T>(pre: Option<T>, post: Option<T>, new: T) -> (T, T)
where
    T: Copy + Clone,
{
    match (pre, post) {
        (Some(pre), Some(post)) => (pre, post),
        (Some(pre), None) => (pre, new),
        (None, Some(post)) => (new, post),
        (None, None) => (new, new),
    }
}

/// Resolve the pre- and post-transaction state of a single account.
///
/// geth's prestate tracer in diff mode records, in `post`, only the fields a
/// transaction actually changed. A field present in `pre` but absent from
/// `post` was accessed but left unchanged, so its post value equals its pre
/// value — those fields are filled in here from `pre`. An account absent from
/// `post` entirely was self-destructed, and its post state is correctly empty
/// (so a destroyed account's `to_value` resolves to zero, not to its pre
/// value). Storage is intentionally left untouched: issue #245 scopes this to
/// balance, nonce, and code, and `add_storages` already diffs slot-by-slot.
fn resolve_pre_post(
    pre: Option<&AccountState>,
    post: Option<&AccountState>,
) -> (AccountState, AccountState) {
    match (pre, post) {
        (Some(pre), Some(post)) => {
            let mut merged = post.clone();
            merged.balance = merged.balance.or(pre.balance);
            merged.nonce = merged.nonce.or(pre.nonce);
            if merged.code.is_none() {
                merged.code = pre.code.clone();
            }
            (pre.clone(), merged)
        }
        (Some(pre), None) => (pre.clone(), AccountState::default()),
        (None, Some(post)) => (AccountState::default(), post.clone()),
        (None, None) => (AccountState::default(), AccountState::default()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unchanged_field_absent_from_post_inherits_pre_value() {
        // Regression (issue #245): geth's diff-mode `post` omits fields the
        // transaction did not change; the post value of an omitted field must
        // equal its pre value, not a zero default.
        let pre =
            AccountState { balance: Some(U256::from(100)), nonce: Some(7), ..Default::default() };
        // the tx bumped the nonce but left the balance untouched
        let post = AccountState { nonce: Some(8), ..Default::default() };

        let (resolved_pre, resolved_post) = resolve_pre_post(Some(&pre), Some(&post));

        assert_eq!(resolved_pre.balance, Some(U256::from(100)));
        assert_eq!(resolved_post.balance, Some(U256::from(100)), "unchanged balance");
        assert_eq!(resolved_post.nonce, Some(8), "changed nonce preserved");
    }

    #[test]
    fn destroyed_account_has_empty_post_state() {
        // An account present only in `pre` was self-destructed; its post state
        // is empty, so to_value resolves to zero rather than the pre value.
        let pre = AccountState {
            balance: Some(U256::from(100)),
            nonce: Some(7),
            code: Some(Bytes::from_static(&[1, 2, 3])),
            ..Default::default()
        };

        let (_, resolved_post) = resolve_pre_post(Some(&pre), None);

        assert_eq!(resolved_post.balance, None);
        assert_eq!(resolved_post.nonce, None);
        assert_eq!(resolved_post.code, None);
    }

    #[test]
    fn changed_field_keeps_post_value() {
        let pre = AccountState { balance: Some(U256::from(100)), ..Default::default() };
        let post = AccountState { balance: Some(U256::from(250)), ..Default::default() };

        let (_, resolved_post) = resolve_pre_post(Some(&pre), Some(&post));

        assert_eq!(resolved_post.balance, Some(U256::from(250)));
    }
}
