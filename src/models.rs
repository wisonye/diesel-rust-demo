use crate::schema::users;

#[derive(Queryable, Debug)]
pub struct User {
    // The `id -> Nullable<Int4>` is an auto increased primary id field,
    // but we want optional.
    pub id: Option<i32>,

    pub name: String,
    pub is_male: bool,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: Option<i32>,
    pub name: &'a str,
    pub is_male: bool,
}
