/**
 * Database objects.
 */
use diesel::prelude::*;

// Taxonomy unit database object.
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::taxonomy::dao::taxonomic_units)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TaxonomicUnit {
    pub tsn: i32,
    pub complete_name: String,
    pub parent_tsn: i32
}

diesel::table! {
    taxonomic_units (tsn) {
        tsn -> Int4,
        complete_name -> Varchar,
        parent_tsn -> Int4
    }
}
