///
/// Common list request object.
///
pub struct TaxonomyListRequest {
    pub start_index: i64,
    pub number_of_elements: i64,
}

impl TaxonomyListRequest {
    pub fn new(start_index: i64, number_of_elements: i64) -> TaxonomyListRequest {
        TaxonomyListRequest {
            start_index: start_index,
            number_of_elements: number_of_elements,
        }
    }
}

pub struct TaxonomyGetRequest {
    pub tsn: i32,
}

impl TaxonomyGetRequest {
    pub fn new(tsn: i32) -> TaxonomyGetRequest {
        TaxonomyGetRequest { tsn: tsn }
    }
}

///
/// List response object.
///
pub struct TaxonomyListResponse {
    pub start_index: i64,
    pub number_of_elements: i64,
    pub has_more_elements: bool,
    pub elements: Vec<TaxonomyListElement>,
}

impl TaxonomyListResponse {
    pub fn new(
        start_index: i64,
        number_of_elements: i64,
        queried_number_of_elements: i64,
        queried_elements: Vec<TaxonomyListElement>,
    ) -> TaxonomyListResponse {
        let queried_elements_length = queried_elements.len() as i64;
        let vec: Vec<TaxonomyListElement>;
        if queried_number_of_elements > queried_elements.len() as i64 {
            vec = queried_elements;
        } else {
            vec = queried_elements
                .into_iter()
                .take(number_of_elements as usize)
                .collect();
        }
        TaxonomyListResponse {
            start_index: start_index,
            number_of_elements: vec.len() as i64,
            has_more_elements: queried_number_of_elements == queried_elements_length,
            elements: vec,
        }
    }
}

pub struct TaxonomyListElement {
    pub tsn: i32,
    pub name: String,
}

impl TaxonomyListElement {
    pub fn new(tsn: i32, name: String) -> TaxonomyListElement {
        TaxonomyListElement {
            tsn: tsn,
            name: name,
        }
    }
}

pub struct TaxonomyGetResponse {
    pub tsn: i32,
    pub name: String,
}

impl TaxonomyGetResponse {
    pub fn new(tsn: i32, name: String) -> TaxonomyGetResponse {
        TaxonomyGetResponse {
            tsn: tsn,
            name: name,
        }
    }
}
