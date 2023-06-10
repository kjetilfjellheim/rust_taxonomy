use serde::Serialize;

use crate::taxonomy::model::{ListResponse, Longname};


#[derive(Serialize)]
pub struct LongnameResponseType {
    pub longnames: Vec<LongnameType>,
    pub pagination: PaginationType
}

#[derive(Serialize)]
pub struct LongnameType {
    tsn: i32,
    completename: String
}

#[derive(Serialize)]
pub struct PaginationType {
    start_index: i64,
    number_of_elements: i64,
    has_more_elements: bool
}

impl From<ListResponse<Longname>> for LongnameResponseType {

    fn from(list_response: ListResponse<Longname>) -> Self {
        let mut vec = Vec::new();

        for element in list_response.elements {
            vec.push(LongnameType::from(element));
        }

        LongnameResponseType {
            pagination : PaginationType {
                start_index: list_response.start_index,
                number_of_elements: list_response.number_of_elements,
                has_more_elements: list_response.has_more_elements,
            },
            longnames: vec
        }
    }
}

impl From<Longname> for LongnameType {
    fn from(longname: Longname) -> Self {
        LongnameType {
            tsn: longname.tsn,
            completename: longname.completename.clone(),
        }
    }
}
