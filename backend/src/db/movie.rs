// !important to use the same order of columns that we have on `schema.rs`
use crate::schema::movies;
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Movie {
    pub id: i32,
    pub owner_id: i32,
    pub name: String,
    pub seen: bool,
}

#[derive(Insertable)]
#[table_name = "movies"]
pub struct NewMovie<'a> {
    pub name: &'a str,
    pub seen: &'a bool,
    pub owner_id: &'a i32,
}

pub fn create_movie<'a>(
    conn: &PgConnection,
    name: &'a str,
    seen: &'a bool,
    owner_id: &'a i32,
) -> Movie {
    let new_movie = NewMovie {
        name: name,
        seen: seen,
        owner_id: owner_id,
    };

    diesel::insert_into(movies::table)
        .values(&new_movie)
        .get_result(conn)
        .expect("Error saving new post")
}
