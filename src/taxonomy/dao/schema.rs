/**
 * Database objects.
 */
use diesel::prelude::*;

///
/// Longname database object.
///
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::taxonomy::dao::taxonomic_units)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TaxonomicUnit {
    pub tsn: i32,
    pub complete_name: String,
}

diesel::table! {
    taxonomic_units (tsn) {
        tsn -> Int4,
        complete_name -> Varchar,
    }
}
