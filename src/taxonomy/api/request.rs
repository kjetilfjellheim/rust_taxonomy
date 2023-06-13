// Models used to represent the request objects used in the api.

use serde::Deserialize;

/// Model used to represent input request to list taxonomy service.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxonomyListRequestQuery {
    pub start_index: Option<i64>,
    pub page_size: Option<i64>,
}
