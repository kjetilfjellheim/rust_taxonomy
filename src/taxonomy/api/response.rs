use serde::Serialize;

use crate::taxonomy::dao::TaxonomicUnit;
use crate::taxonomy::model::ListResponse;

///
/// Longname response object from the api
///
#[derive(Serialize)]
pub struct LongnameResponseType {
    pub longnames: Vec<LongnameType>,
    pub pagination: PaginationType,
}

///
/// Single Longname used in the list service.
///
#[derive(Serialize)]
pub struct LongnameType {
    tsn: i32,
    completename: String,
}

///
/// Common pagination response object.
///
#[derive(Serialize)]
pub struct PaginationType {
    start_index: i64,
    number_of_elements: i64,
    has_more_elements: bool,
}

///
/// Converter from List response object to longname list object response.
///
impl From<ListResponse<TaxonomicUnit>> for LongnameResponseType {
    fn from(list_response: ListResponse<TaxonomicUnit>) -> Self {
        let mut vec = Vec::new();

        for element in list_response.elements {
            vec.push(LongnameType::from(element));
        }

        LongnameResponseType {
            pagination: PaginationType {
                start_index: list_response.start_index,
                number_of_elements: list_response.number_of_elements,
                has_more_elements: list_response.has_more_elements,
            },
            longnames: vec,
        }
    }
}

///
///  Convert single Longname db object to response object.
///
impl From<TaxonomicUnit> for LongnameType {
    fn from(longname: TaxonomicUnit) -> Self {
        LongnameType {
            tsn: longname.tsn,
            completename: longname.complete_name.clone(),
        }
    }
}
