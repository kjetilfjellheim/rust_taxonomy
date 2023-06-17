use serde::Serialize;

use crate::taxonomy::model::{
    TaxonomyGetChild, TaxonomyGetResponse, TaxonomyListElement, TaxonomyListResponse,
};

///
/// Taxonomy list response object from the api.
///
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxonomyListResponseType {
    pub taxonomy: Vec<TaxonomyElementType>,
    pub pagination: PaginationType,
}

///
/// Single taxonomy element. Used in both list and single get services.
///
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxonomyElementType {
    tsn: i32,
    name: String,
    kingdom_name: String,
    rank_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_tsn: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<TaxonomyChildElementType>>,
}

///
/// Taxonomy get child element.
///
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxonomyChildElementType {
    tsn: i32,
    name: String,
}

impl From<TaxonomyGetResponse> for TaxonomyElementType {
    fn from(response: TaxonomyGetResponse) -> Self {
        TaxonomyElementType {
            tsn: response.tsn,
            name: response.name,
            kingdom_name: response.kingdom_name,
            rank_name: response.rank_name,
            parent_tsn: response.parent_tsn,
            parent_name: response.parent_name,
            children: match response.children {
                Some(children) => Some(
                    children
                        .iter()
                        .map(|child| TaxonomyChildElementType::from(child))
                        .collect(),
                ),
                None => None,
            },
        }
    }
}

impl From<&TaxonomyGetChild> for TaxonomyChildElementType {
    fn from(child_element: &TaxonomyGetChild) -> Self {
        TaxonomyChildElementType {
            tsn: child_element.tsn,
            name: child_element.name.clone(),
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
            taxonomy: vec,
        }
    }
}

impl From<TaxonomyListElement> for TaxonomyElementType {
    fn from(list_element: TaxonomyListElement) -> Self {
        TaxonomyElementType {
            tsn: list_element.tsn,
            name: list_element.name.clone(),
            kingdom_name: list_element.kingdom_name,
            rank_name: list_element.rank_name,
            parent_tsn: None,
            parent_name: None,
            children: None,
        }
    }
}
