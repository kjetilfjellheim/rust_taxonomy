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
    /// Optional on what to sort on. Default Tsn.
    pub sort: Option<TaxonomyListSort>,
    /// Optional on what to sorting. Default asc.
    pub order: Option<TaxonomyListOrder>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TaxonomyListSort {
    Tsn,
    Name,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TaxonomyListOrder {
    Asc,
    Desc
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxonomyListRequestBody {
    pub name: Option<String>,
    pub kingdom_name: Option<String>,
    pub rank_name: Option<String>
}
