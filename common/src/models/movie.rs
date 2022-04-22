use serde::{Deserialize, Serialize};

// Owner data
#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Movie {
    pub id: i32,
    pub owner_id: i32,
    pub name: String,
    pub seen: bool,
}

// Movie query
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct MovieRequest {
    pub name: String,
    pub seen: bool,
    pub owner_id: i32,
}

// Movie response
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct MovieResponse {
    pub id: i32,
    pub owner_id: i32,
    pub name: String,
    pub seen: bool,
}

impl MovieResponse {
    pub fn of(movie: Movie) -> MovieResponse {
        MovieResponse {
            id: movie.id,
            owner_id: movie.owner_id,
            name: movie.name,
            seen: movie.seen,
        }
    }
}
