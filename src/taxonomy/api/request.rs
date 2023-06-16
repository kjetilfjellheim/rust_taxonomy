/// Models used to represent the request objects used in the api.
use serde::Deserialize;

/// Model used to represent input request to list taxonomy service.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxonomyListRequestQuery {
    /// Optional start index. Default 0.
    pub start_index: Option<i64>,
    /// Optional page size. Default 500.
    pub page_size: Option<i64>,
}
