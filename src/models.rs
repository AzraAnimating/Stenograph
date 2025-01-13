use diesel::prelude::*;


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::file)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct File {
    pub id: String, 
    pub name: String,
    pub datatype: i32,
}
