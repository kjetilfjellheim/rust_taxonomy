use crate::taxonomy::dao::taxonomic_units::dsl::taxonomic_units as taxonomic_units_dsl;
use crate::taxonomy::dao::TaxonomicUnit;
use diesel::prelude::*;
use crate::taxonomy::dao::schema::taxonomic_units as taxonomic_units_schema;

///
/// Find taxonomies elements using start_index and page_size.
///
pub fn find_taxonomies(
    connection: &mut PgConnection,
    start_index: i64,
    page_size: i64,
) -> Result<Vec<TaxonomicUnit>, diesel::result::Error> {
    taxonomic_units_dsl
        .limit(page_size + 1)
        .offset(start_index)
        .select(TaxonomicUnit::as_select())
        .load(connection)
}

///
/// Find taxonomies elements us.
///
pub fn find_child_taxonomies(
    connection: &mut PgConnection,
    parent_tsn: i32
) -> Result<Vec<TaxonomicUnit>, diesel::result::Error> {
    taxonomic_units_dsl
        .select(TaxonomicUnit::as_select())
        .filter(taxonomic_units_schema::parent_tsn.eq(parent_tsn))
        .load(connection)
}

///
/// Query single taxonomy element.
///
pub fn find_taxonomy(
    connection: &mut PgConnection,
    tsn: i32,
) -> Result<TaxonomicUnit, diesel::result::Error> {
    // Query tsn
    taxonomic_units_dsl
        .select(TaxonomicUnit::as_select())
        .find(tsn)
        .first(connection)
}
