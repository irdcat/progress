use serde::Serialize;

use super::schema::test;

#[derive(Serialize, Queryable)]
pub struct Test {
    pub id: String,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "test"]
pub struct NewTest<'a> {
    pub id: &'a str,
    pub name: &'a str,
}
