use crate::db::establish_connection;
use crate::db::movie::{movie_create, Movie};
use crate::schema::movies::dsl::movies;
use diesel::prelude::*;

pub fn list_movies() -> Result<Vec<Movie>, diesel::result::Error> {
    let connection = establish_connection();
    movies.load::<Movie>(&connection)
}

pub fn create_movie(
    name: &String,
    seen: &bool,
    owner_id: &i32,
) -> Result<Movie, diesel::result::Error> {
    let connection = establish_connection();
    movie_create(&connection, name.as_str(), seen, owner_id)
}
