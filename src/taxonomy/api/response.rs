use serde::Serialize;

use crate::taxonomy::model::{ TaxonomyListResponse, TaxonomyListElement, TaxonomyGetResponse } ;

///
/// Longname response object from the api
///
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxonomyListResponseType {
    pub tsn: Vec<TaxonomyElementType>,
    pub pagination: PaginationType,
}

///
/// Single Longname used in the list service.
///
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxonomyElementType {
    tsn: i32,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_tsn: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<TaxonomyChildElementType>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxonomyChildElementType {
    tsn: i32,
    name: String,
}

///
/// Converter from GEt response object to tsn response object.
///
impl From<TaxonomyGetResponse> for TaxonomyElementType {
    fn from(response: TaxonomyGetResponse) -> Self {
        TaxonomyElementType {
            tsn: response.tsn,
            name: response.name,
            parent_tsn: None,
            parent_name: None,
            children: None,
        }
    }
}

///
/// Common pagination response object.
///
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationType {
    start_index: i64,
    number_of_elements: i64,
    has_more_elements: bool,
}

///
/// Converter from List response object to longname list object response.
///
impl From<TaxonomyListResponse> for TaxonomyListResponseType {
    fn from(list_response: TaxonomyListResponse) -> Self {
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
impl From<TaxonomyListElement> for TaxonomyElementType {
    fn from(list_element: TaxonomyListElement) -> Self {
        TaxonomyElementType {
            tsn: list_element.tsn,
            name: list_element.name.clone(),
            parent_tsn: None,
            parent_name: None,
            children: None,
        }
    }
}
