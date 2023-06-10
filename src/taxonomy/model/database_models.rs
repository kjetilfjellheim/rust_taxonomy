use diesel::{ prelude::* };

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::taxonomy::model::schema::longnames)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Longname {
    pub tsn: i32,
    pub completename: String
}
