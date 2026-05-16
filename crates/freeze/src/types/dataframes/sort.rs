use polars::prelude::*;

use crate::types::{CollectError, Table};

pub(crate) trait SortableDataFrame {
    fn sort_by_schema(self, schema: &Table) -> Self;
}

impl SortableDataFrame for Result<DataFrame, CollectError> {
    fn sort_by_schema(self, schema: &Table) -> Self {
        match (self, &schema.sort_columns) {
            (Ok(df), Some(sort_columns)) => df
                // maintain_order = true keeps the sort stable: rows with equal
                // sort keys retain their (deterministic) input order. polars
                // 0.53's default sort is unstable and orders ties
                // non-deterministically — see ROADMAP P1-10.
                .sort(sort_columns, SortMultipleOptions::default().with_maintain_order(true))
                .map_err(CollectError::PolarsError),
            (df, _) => df,
        }
    }
}
