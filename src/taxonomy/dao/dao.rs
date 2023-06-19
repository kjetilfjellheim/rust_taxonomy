
use crate::taxonomy::dao::schema::v_taxonomy as taxonomic_units_schema;
use crate::taxonomy::dao::v_taxonomy::dsl::v_taxonomy as taxonomic_units_dsl;
use crate::taxonomy::dao::TaxonomicUnit;
use crate::taxonomy::model::{TaxonomyListSort, TaxonomyListOrder};
use diesel::prelude::*;

///
/// Find taxonomies elements using start_index and page_size.
///
pub fn find_taxonomies(
    connection: &mut PgConnection,
    start_index: i64,
    page_size: i64,
    taxonomy_list_sort : TaxonomyListSort,
    taxonomy_list_order : TaxonomyListOrder,
    filter_kingdomname: Option<String>,
    filter_rankname: Option<String>,
    filter_name: Option<String>
) -> Result<Vec<TaxonomicUnit>, diesel::result::Error> {

    let mut query = taxonomic_units_schema::table.into_boxed();

    if let Some(filter_kingdomname) = filter_kingdomname {
        let mut str = filter_kingdomname.clone();
        str.insert(0, '%');
        str.push('%');
        query = query.filter(taxonomic_units_schema::kingdom_name.ilike(str));
    }

    if let Some(filter_rankname) = filter_rankname {
        let mut str = filter_rankname.clone();
        str.insert(0, '%');
        str.push('%');
        query = query.filter(taxonomic_units_schema::rank_name.ilike(str));
    }

    if let Some(filter_name) = filter_name {
        let mut str = filter_name.clone();
        str.insert(0, '%');
        str.push('%');
        query = query.filter(taxonomic_units_schema::complete_name.ilike(str));
    }

    let query = match taxonomy_list_sort {
        TaxonomyListSort::Name => {
            match taxonomy_list_order {
                TaxonomyListOrder::Asc => { query.order(taxonomic_units_schema::complete_name.asc()) },
                TaxonomyListOrder::Desc => { query.order(taxonomic_units_schema::complete_name.desc()) }
            }
        },
        TaxonomyListSort::Tsn => {
            match taxonomy_list_order {
                TaxonomyListOrder::Asc => { query.order(taxonomic_units_schema::tsn.asc()) },
                TaxonomyListOrder::Desc => { query.order(taxonomic_units_schema::tsn.desc()) }
            }
        }

    };

    query
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
    parent_tsn: i32,
) -> Result<Vec<TaxonomicUnit>, diesel::result::Error> {
    taxonomic_units_dsl
        .select(TaxonomicUnit::as_select())
        .order_by(taxonomic_units_schema::parent_tsn.asc())
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
