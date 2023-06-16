use crate::taxonomy::dao::TaxonomicUnit;
use crate::taxonomy::dao::{
    taxonomic_units::dsl::taxonomic_units as taxonomic_units_dsl,
};
use diesel::prelude::*;

///
/// Find taxonomies elements using start_index and page_size.
///
pub fn find_taxonomies(
    connection: &mut PgConnection,
    start_index: i64,
    page_size: i64
) -> Result<Vec<TaxonomicUnit>, diesel::result::Error> {
    taxonomic_units_dsl
        .limit(page_size + 1)
        .offset(start_index)
        .select(TaxonomicUnit::as_select())
        .load(connection)
}

///
/// Query single taxonomy element.
///
pub fn find_taxonomy(
    connection: &mut PgConnection,
    tsn: i32
) -> Result<TaxonomicUnit, diesel::result::Error> {
    // Query tsn
    taxonomic_units_dsl
        .select(TaxonomicUnit::as_select())
        .find(tsn)
        .first(connection)
}
