use polars::prelude::*;

use crate::types::{CollectError, Table};

pub(crate) trait SortableDataFrame {
    fn sort_by_schema(self, schema: &Table) -> Self;
}

impl SortableDataFrame for Result<DataFrame, CollectError> {
    fn sort_by_schema(self, schema: &Table) -> Self {
        match (self, &schema.sort_columns) {
            (Ok(df), Some(sort_columns)) => {
                // Sort only by columns actually present in the dataframe. A
                // user can exclude a column that is part of the dataset's
                // default_sort (issue #221); sorting by a missing column
                // otherwise fails and zeroes the output. If every sort column
                // was excluded, leave the frame in its (deterministic)
                // collection order.
                let present: Vec<String> = sort_columns
                    .iter()
                    .filter(|name| df.column(name.as_str()).is_ok())
                    .cloned()
                    .collect();
                if present.is_empty() {
                    return Ok(df);
                }
                // maintain_order = true keeps the sort stable: rows with equal
                // sort keys retain their (deterministic) input order. polars
                // 0.53's default sort is unstable and orders ties
                // non-deterministically — see ROADMAP P1-10.
                df.sort(&present, SortMultipleOptions::default().with_maintain_order(true))
                    .map_err(CollectError::PolarsError)
            }
            (df, _) => df,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ColumnEncoding, Datatype, U256Type};

    #[test]
    fn excluding_a_sort_column_keeps_all_rows() {
        // Regression (issue #221): a user excluded `create_index`, which is
        // part of the `contracts` default_sort. Sorting by the now-absent
        // column must not fail and zero the output.
        let df = df!("block_number" => &[3u32, 1, 2]).unwrap();
        let schema = Datatype::Contracts
            .table_schema(
                &[U256Type::Binary],
                &ColumnEncoding::Hex,
                &None,
                &None,
                &None,
                Some(vec!["block_number".to_string(), "create_index".to_string()]),
                None,
            )
            .unwrap();

        let input: Result<DataFrame, CollectError> = Ok(df);
        let sorted =
            input.sort_by_schema(&schema).expect("excluding a sort column must not fail the sort");

        // every row retained, and the still-present sort column was applied
        let expected = df!("block_number" => &[1u32, 2, 3]).unwrap();
        assert!(sorted.equals(&expected), "got {:?}", sorted);
    }
}
