/**
 * Database objects.
 */
use diesel::prelude::*;

// Taxonomy unit database object.
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::taxonomy::dao::v_taxonomy)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TaxonomicUnit {
    pub tsn: i32,
    pub complete_name: String,
    pub parent_tsn: i32,
    pub kingdom_name: String,
    pub rank_name: String,
}

diesel::table! {
    v_taxonomy (tsn) {
        tsn -> Int4,
        complete_name -> Varchar,
        parent_tsn -> Int4,
        kingdom_name -> Char,
        rank_name -> Char
    }
}
