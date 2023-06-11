///
/// Common list request object.
///
pub struct ListRequest {
    pub start_index: i64,
    pub number_of_elements: i64,
}

impl ListRequest {
    pub fn new(start_index: i64, number_of_elements: i64) -> ListRequest {
        ListRequest {
            start_index: start_index,
            number_of_elements: number_of_elements,
        }
    }
}

///
/// List response object.
///
pub struct ListResponse<T> {
    pub start_index: i64,
    pub number_of_elements: i64,
    pub has_more_elements: bool,
    pub elements: Vec<T>,
}

impl<T> ListResponse<T> {
    pub fn new(
        start_index: i64,
        number_of_elements: i64,
        queried_number_of_elements: i64,
        queried_elements: Vec<T>,
    ) -> ListResponse<T> {

        let queried_elements_length = queried_elements.len() as i64;
        let vec: Vec<T>;
        if queried_number_of_elements > queried_elements.len() as i64 {
            vec = queried_elements;
        } else {
            vec = queried_elements
                .into_iter()
                .take(number_of_elements as usize)
                .collect();
        }
        ListResponse {
            start_index: start_index,
            number_of_elements: vec.len() as i64,
            has_more_elements: queried_number_of_elements == queried_elements_length,
            elements: vec,
        }
    }
}
