use serde::Serialize;

use crate::taxonomy::dao::TaxonomicUnit;
use crate::taxonomy::model::ListResponse;

///
/// Longname response object from the api
///
#[derive(Serialize)]
pub struct TaxonomyListResponseType {
    pub tsn: Vec<TaxonomyElementType>,
    pub pagination: PaginationType,
}

///
/// Single Longname used in the list service.
///
#[derive(Serialize)]
pub struct TaxonomyElementType {
    tsn: i32,
    name: String,
    parent_tsn: Option<i32>,
    parent_name: Option<String>,
    children: Option<Vec<TaxonomyChildElementType>>
}

#[derive(Serialize)]
pub struct TaxonomyChildElementType {
    tsn: i32,
    name: String,
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
impl From<ListResponse<TaxonomicUnit>> for TaxonomyListResponseType {
    fn from(list_response: ListResponse<TaxonomicUnit>) -> Self {
        let mut vec = Vec::new();

        for element in list_response.elements {
            vec.push(TaxonomyElementType::from(element));
        }

        TaxonomyListResponseType {
            pagination: PaginationType {
                start_index: list_response.start_index,
                number_of_elements: list_response.number_of_elements,
                has_more_elements: list_response.has_more_elements,
            },
            tsn: vec,
        }
    }
}

///
///  Convert single Longname db object to response object.
///
impl From<TaxonomicUnit> for TaxonomyElementType {
    fn from(longname: TaxonomicUnit) -> Self {
        TaxonomyElementType {
            tsn: longname.tsn,
            name: longname.complete_name.clone(),
            parent_tsn: None,
            parent_name: None,
            children: None
        }
    }
}
