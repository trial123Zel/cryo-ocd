//! Incremental dataset consolidation.
//!
//! cryo writes one output file per collected chunk, which leaves large datasets
//! spread across thousands of small files. This module merges those files into
//! fewer, larger, aligned files once a chunk boundary is complete — a completed
//! boundary will never need rewriting, so it is safe to merge eagerly.
//!
//! Implements roadmap task P4-11. The incremental, tiered scheme follows
//! upstream issue paradigmxyz/cryo#29 (credit: @banteg); the implementation is
//! written fresh for cryo-ocd in Rust.

use crate::{
    dataframes, err, BlockChunk, CollectError, Datatype, Dim, FileOutput, Partition, Query, SubDir,
};
use polars::prelude::*;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

/// consolidation tiers, as multiples of the base chunk size (largest first)
const TIER_MULTIPLIERS: [u64; 3] = [1000, 100, 10];

/// summary of a consolidation pass
#[derive(Debug, Default, Clone)]
pub(crate) struct ConsolidateSummary {
    /// number of consolidated files written
    pub(crate) n_files_written: u64,
    /// number of source files merged away
    pub(crate) n_files_merged: u64,
}

/// an output file on disk together with its parsed inclusive block range
struct ChunkFile {
    path: PathBuf,
    start: u64,
    end: u64,
}

/// whether a query's output is block-range partitioned (the only layout
/// consolidation supports)
fn is_block_partitioned(query: &Query) -> bool {
    matches!(query.partitioned_by.as_slice(), [Dim::BlockNumber])
}

/// the run's configured chunk size — the largest block-chunk in the query.
/// taken from the query rather than from file spans so that a ragged final
/// chunk (smaller than the chunk size) cannot lower it
fn base_chunk_size(query: &Query) -> u64 {
    query
        .partitions
        .iter()
        .filter_map(|partition| partition.block_numbers.as_ref())
        .flatten()
        .map(|chunk| match chunk {
            BlockChunk::Range(start, end) => end - start + 1,
            BlockChunk::Numbers(numbers) => numbers.len() as u64,
        })
        .max()
        .unwrap_or(0)
}

/// consolidate every datatype's output files into larger aligned files
pub(crate) fn consolidate_query(
    query: &Query,
    sink: &FileOutput,
    verbose: bool,
) -> Result<ConsolidateSummary, CollectError> {
    let mut summary = ConsolidateSummary::default();

    if !is_block_partitioned(query) {
        if verbose {
            println!("consolidation skipped: only block-range partitioning is supported");
        }
        return Ok(summary)
    }
    if sink.format.as_str() != "parquet" {
        if verbose {
            println!("consolidation skipped: only parquet output is supported");
        }
        return Ok(summary)
    }

    let base = base_chunk_size(query);
    if base == 0 {
        return Ok(summary)
    }

    for meta_datatype in query.datatypes.iter() {
        for datatype in meta_datatype.datatypes().into_iter() {
            if !query.schemas.contains_key(&datatype) {
                continue
            }
            let dt = consolidate_datatype(&datatype, sink, base, verbose)?;
            summary.n_files_written += dt.n_files_written;
            summary.n_files_merged += dt.n_files_merged;
        }
    }
    Ok(summary)
}

/// inclusive block ranges of all on-disk output files for a datatype; used by
/// the freeze pipeline so re-runs recognise already-consolidated ranges
pub(crate) fn existing_block_ranges(
    query: &Query,
    sink: &FileOutput,
    datatype: &Datatype,
) -> Result<Vec<(u64, u64)>, CollectError> {
    if !is_block_partitioned(query) {
        return Ok(Vec::new())
    }
    let dir = datatype_dir(sink, datatype);
    Ok(scan_chunk_files(&dir, sink, datatype)?.into_iter().map(|f| (f.start, f.end)).collect())
}

/// whether a base partition's block range is fully covered by an existing file
pub(crate) fn partition_covered(partition: &Partition, ranges: &[(u64, u64)]) -> bool {
    let Some(chunks) = partition.block_numbers.as_ref() else { return false };
    // only a single contiguous range can be coverage-checked
    let [BlockChunk::Range(start, end)] = chunks.as_slice() else { return false };
    ranges.iter().any(|&(rs, re)| rs <= *start && re >= *end)
}

fn consolidate_datatype(
    datatype: &Datatype,
    sink: &FileOutput,
    base: u64,
    verbose: bool,
) -> Result<ConsolidateSummary, CollectError> {
    let mut summary = ConsolidateSummary::default();
    let dir = datatype_dir(sink, datatype);
    let mut files = scan_chunk_files(&dir, sink, datatype)?;
    if files.len() < 2 {
        return Ok(summary)
    }

    // process the largest tier first so smaller-tier passes see merged files
    for mult in TIER_MULTIPLIERS {
        let tier = base.saturating_mul(mult);
        consolidate_tier(&mut files, tier, datatype, sink, verbose, &mut summary)?;
    }
    Ok(summary)
}

fn consolidate_tier(
    files: &mut Vec<ChunkFile>,
    tier: u64,
    datatype: &Datatype,
    sink: &FileOutput,
    verbose: bool,
    summary: &mut ConsolidateSummary,
) -> Result<(), CollectError> {
    // group files by the aligned tier-block they fall entirely within
    let mut groups: BTreeMap<u64, Vec<usize>> = BTreeMap::new();
    for (i, f) in files.iter().enumerate() {
        let block = f.start / tier;
        if f.start >= block * tier && f.end < (block + 1) * tier {
            groups.entry(block).or_default().push(i);
        }
    }

    let mut consumed: Vec<usize> = Vec::new();
    let mut merged_files: Vec<ChunkFile> = Vec::new();

    for (block, mut idxs) in groups {
        let tier_start = block * tier;
        let tier_end = tier_start + tier - 1;
        idxs.sort_by_key(|&i| files[i].start);

        // a single file already covering exactly this tier-block: nothing to do
        if idxs.len() == 1 && files[idxs[0]].start == tier_start && files[idxs[0]].end == tier_end {
            continue
        }

        // do the group's files tile [tier_start, tier_end] exactly, with no gaps?
        let mut cursor = tier_start;
        let mut complete = true;
        for &i in idxs.iter() {
            if files[i].start != cursor {
                complete = false;
                break
            }
            cursor = files[i].end + 1;
        }
        if !complete || cursor != tier_end + 1 {
            continue
        }

        // merge: write the consolidated file first, then delete the sources
        let src_paths: Vec<PathBuf> = idxs.iter().map(|&i| files[i].path.clone()).collect();
        let out_path = consolidated_path(sink, datatype, tier_start, tier_end);
        merge_files(&src_paths, &out_path, sink)?;
        for path in src_paths.iter() {
            if path != &out_path {
                std::fs::remove_file(path)
                    .map_err(|e| err(&format!("could not remove {}: {}", path.display(), e)))?;
            }
        }

        if verbose {
            println!(
                "consolidated {} {} files into {}",
                idxs.len(),
                datatype.name(),
                out_path.file_name().unwrap_or_default().to_string_lossy(),
            );
        }
        summary.n_files_written += 1;
        summary.n_files_merged += idxs.len() as u64;
        consumed.extend(idxs);
        merged_files.push(ChunkFile { path: out_path, start: tier_start, end: tier_end });
    }

    // rebuild the file list: drop consumed files, add the consolidated ones
    consumed.sort_unstable();
    let mut rebuilt: Vec<ChunkFile> = Vec::new();
    for (i, f) in files.drain(..).enumerate() {
        if consumed.binary_search(&i).is_err() {
            rebuilt.push(f);
        }
    }
    rebuilt.extend(merged_files);
    *files = rebuilt;
    Ok(())
}

/// read the source parquet files in order and write them as one file
fn merge_files(
    src_paths: &[PathBuf],
    out_path: &Path,
    sink: &FileOutput,
) -> Result<(), CollectError> {
    // src_paths are in ascending block order and each file is already sorted,
    // so concatenating them yields a globally sorted dataframe with no re-sort
    let mut merged: Option<DataFrame> = None;
    for path in src_paths.iter() {
        let file = std::fs::File::open(path)
            .map_err(|e| err(&format!("could not open {}: {}", path.display(), e)))?;
        let df = ParquetReader::new(file)
            .finish()
            .map_err(|e| err(&format!("could not read {}: {}", path.display(), e)))?;
        merged = Some(match merged {
            None => df,
            Some(mut acc) => {
                acc.vstack_mut(&df)
                    .map_err(|e| err(&format!("could not merge dataframes: {}", e)))?;
                acc
            }
        });
    }
    let mut merged = merged.ok_or_else(|| err("no files to merge"))?;
    dataframes::df_to_file(&mut merged, out_path, sink)
        .map_err(|_| err("could not write consolidated file"))
}

/// directory holding a datatype's output files (mirrors `FileOutput::get_path`)
fn datatype_dir(sink: &FileOutput, datatype: &Datatype) -> PathBuf {
    let mut dir = sink.output_dir.clone();
    for subdir in sink.subdirs.iter() {
        let piece = match subdir {
            SubDir::Network => sink.prefix.clone(),
            SubDir::Datatype => match &sink.suffix {
                Some(suffix) => datatype.name() + "__" + suffix,
                None => datatype.name(),
            },
            SubDir::Custom(custom) => custom.clone(),
        };
        dir = dir.join(piece);
    }
    dir
}

/// output path for a consolidated [start, end] block range (mirrors the
/// filename built by `FileOutput::get_path`)
fn consolidated_path(sink: &FileOutput, datatype: &Datatype, start: u64, end: u64) -> PathBuf {
    let stub = format!("{:0>8}_to_{:0>8}", start, end);
    let ext = sink.format.as_str();
    let filename = match &sink.suffix {
        Some(suffix) => {
            format!("{}__{}__{}__{}.{}", sink.prefix, datatype.name(), suffix, stub, ext)
        }
        None => format!("{}__{}__{}.{}", sink.prefix, datatype.name(), stub, ext),
    };
    datatype_dir(sink, datatype).join(filename)
}

/// enumerate a datatype's chunk files and parse their block ranges
fn scan_chunk_files(
    dir: &Path,
    sink: &FileOutput,
    datatype: &Datatype,
) -> Result<Vec<ChunkFile>, CollectError> {
    let mut files = Vec::new();
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return Ok(files), // directory absent => nothing to consolidate
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue
        }
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else { continue };
        if let Some((start, end)) = parse_chunk_filename(name, sink, datatype) {
            files.push(ChunkFile { path, start, end });
        }
    }
    Ok(files)
}

/// parse the inclusive block range out of a chunk filename, or `None` if the
/// filename does not belong to this datatype / is not a plain block range
fn parse_chunk_filename(name: &str, sink: &FileOutput, datatype: &Datatype) -> Option<(u64, u64)> {
    let rest = name.strip_suffix(&format!(".{}", sink.format.as_str()))?;
    let rest = rest.strip_prefix(&format!("{}__{}__", sink.prefix, datatype.name()))?;
    let rest = match &sink.suffix {
        Some(suffix) => rest.strip_prefix(&format!("{}__", suffix))?,
        None => rest,
    };
    let (start, end) = rest.split_once("_to_")?;
    let start: u64 = start.parse().ok()?;
    let end: u64 = end.parse().ok()?;
    (end >= start).then_some((start, end))
}
