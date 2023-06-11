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

///
/// Implementation services for the ListRequestQuery struct,
///
impl ListRequestQuery {
    pub fn new(start_index: Option<i64>, page_size: Option<i64>) -> ListRequestQuery {
        ListRequestQuery {
            start_index: start_index,
            page_size: page_size,
        }
    }
}
