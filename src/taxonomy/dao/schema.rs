/**
 * Database objects.
 */

use diesel::prelude::*;

///
/// Longname database object.
///
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::taxonomy::dao::longnames)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Longname {
    pub tsn: i32,
    pub completename: String,
}

diesel::table! {
    longnames (tsn) {
        tsn -> Int4,
        completename -> Varchar,
    }
}
