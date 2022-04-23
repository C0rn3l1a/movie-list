// !important to use the same order of columns that we have on `schema.rs`
use crate::schema::movies;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};
#[derive(Queryable, Serialize, Deserialize, Debug)]
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

pub fn movie_create<'a>(
    conn: &PgConnection,
    name: &'a str,
    seen: &'a bool,
    owner_id: &'a i32,
) -> Result<Movie, Error> {
    let new_movie = NewMovie {
        name: name,
        seen: seen,
        owner_id: owner_id,
    };

    diesel::insert_into(movies::table)
        .values(&new_movie)
        .get_result(conn)
}

#[derive(AsChangeset)]
#[table_name = "movies"]
struct MovieForm<'a> {
    pub name: Option<&'a str>,
    pub seen: Option<&'a bool>,
    pub owner_id: Option<&'a i32>,
}

pub fn movie_update<'a>(
    conn: &PgConnection,
    id: &'a i32,
    name: &'a str,
    seen: &'a bool,
    owner_id: &'a i32,
) -> Result<Movie, Error> {
    let movie_form = MovieForm {
        name: Some(name),
        seen: Some(seen),
        owner_id: Some(owner_id),
    };

    diesel::update(movies::table.find(id))
        .set(movie_form)
        .get_result::<Movie>(conn)
}
