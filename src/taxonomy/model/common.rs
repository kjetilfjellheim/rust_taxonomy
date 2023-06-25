///
/// Internal application model for taxonomy list requests.
///
pub struct TaxonomyListRequest {
    /// Start index starts with 0 to infinite.
    pub start_index: i64,
    /// Max number of elements to return.
    pub number_of_elements: i64,
    /// What to sort for.
    pub taxonomy_list_sort: TaxonomyListSort,
    /// Ascending or descending.
    pub taxonomy_list_order: TaxonomyListOrder,
    pub filter_kingdomname: Option<String>,
    pub filter_rankname: Option<String>,
    pub filter_name: Option<String>
}

impl TaxonomyListRequest {
    pub fn new(
        start_index: i64,
        number_of_elements: i64,
        taxonomy_list_sort: TaxonomyListSort,
        taxonomy_list_order: TaxonomyListOrder,
        filter_kingdomname: Option<String>,
        filter_rankname: Option<String>,
        filter_name: Option<String>
    ) -> TaxonomyListRequest {
        TaxonomyListRequest {
            start_index: start_index,
            number_of_elements: number_of_elements,
            taxonomy_list_sort: taxonomy_list_sort,
            taxonomy_list_order: taxonomy_list_order,
            filter_kingdomname: filter_kingdomname,
            filter_rankname: filter_rankname,
            filter_name: filter_name
        }
    }
}

///
/// Internal application get tsn request.
///
pub struct TaxonomyGetRequest {
    /// Taxonomy identifier.
    pub tsn: i32,
}

impl TaxonomyGetRequest {
    pub fn new(tsn: i32) -> TaxonomyGetRequest {
        TaxonomyGetRequest { tsn: tsn }
    }
}

///
/// Internal Taxonomy list response object.
///
pub struct TaxonomyListResponse {
    /// Start index, always the same as in request.
    pub start_index: i64,
    /// Number of elements returned.
    pub number_of_elements: i64,
    /// Do the list contain more data.
    pub has_more_elements: bool,
    /// Data to be returned.
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

///
/// Internal taxonomy list element,
///
pub struct TaxonomyListElement {
    /// Taxonomy identifier.
    pub tsn: i32,
    /// Taxononomy name.
    pub name: String,
    pub kingdom_name: String,
    pub rank_name: String,
}

impl TaxonomyListElement {
    pub fn new(
        tsn: i32,
        name: String,
        kingdomname: String,
        rank_name: String,
    ) -> TaxonomyListElement {
        TaxonomyListElement {
            tsn: tsn,
            name: name,
            kingdom_name: kingdomname,
            rank_name: rank_name,
        }
    }
}

///
///  Internal taxonomy get response.
///
pub struct TaxonomyGetResponse {
    /// Taxonomy identifier.
    pub tsn: i32,
    /// Taxononomy name.
    pub name: String,
    // Kingdom name
    pub kingdom_name: String,
    // Rank name
    pub rank_name: String,
    /// Parennt tsn
    pub parent_tsn: Option<i32>,
    /// Parent taxonomy name
    pub parent_name: Option<String>,
    /// Children taxonomy
    pub children: Option<Vec<TaxonomyGetChild>>,
}

impl TaxonomyGetResponse {
    pub fn new(
        tsn: i32,
        name: String,
        kingdom_name: String,
        rank_name: String,
        parent_tsn: Option<i32>,
        parent_name: Option<String>,
        children: Option<Vec<TaxonomyGetChild>>,
    ) -> TaxonomyGetResponse {
        TaxonomyGetResponse {
            tsn: tsn,
            name: name,
            kingdom_name: kingdom_name,
            rank_name: rank_name,
            parent_tsn: parent_tsn,
            parent_name: parent_name,
            children: children,
        }
    }
}

///
///  Internal taxonomy get response.
///
pub struct TaxonomyHierarchyResponse {
    pub hierarchy: Vec<TaxonomyHierarchyElement>
}

pub struct TaxonomyHierarchyElement {
    pub tsn: i32,
    pub name: String,
    pub kingdom_name: String,
    pub rank_name: String
}

impl TaxonomyHierarchyElement {
    pub fn new(tsn: i32,
        name: String,
        kingdom_name: String,
        rank_name: String) -> TaxonomyHierarchyElement {
        TaxonomyHierarchyElement {
            tsn: tsn,
            name: name,
            kingdom_name: kingdom_name,
            rank_name: rank_name
        }
    }
}

pub struct TaxonomyGetChild {
    /// Child taxonomy tsn
    pub tsn: i32,
    /// Chhild taxonomy name
    pub name: String,
}

impl TaxonomyGetChild {
    pub fn new(
        tsn: i32,
        name: String,
    ) -> TaxonomyGetChild {
        TaxonomyGetChild {
            tsn: tsn,
            name: name,
        }
    }
}

pub enum TaxonomyListSort {
    Tsn,
    Name,
}

pub enum TaxonomyListOrder {
    Asc,
    Desc
}
