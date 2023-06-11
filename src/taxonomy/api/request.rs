///
/// Models used to represent the request objects used in the api.
///
use serde::Deserialize;

///
/// Model used to represent input request to list services.
///
#[derive(Debug, Deserialize)]
pub struct ListRequestQuery {
    pub start_index: Option<i64>,
    pub page_size: Option<i64>,
}
