use crate::services::movie;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

// Structs

// move to common??
#[derive(Deserialize)]
pub struct PostMovieJson {
    pub name: String,
    pub seen: bool,
    pub owner_id: i32,
}

// Functions

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(get_movies);
    cfg.service(post_movies);
}

#[get("/movies")]
async fn get_movies() -> impl Responder {
    match movie::list_movies() {
        Ok(movies) => HttpResponse::Ok().json(movies),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[post("/movies")]
async fn post_movies(body: web::Json<PostMovieJson>) -> impl Responder {
    match movie::create_movie(&body.name, &body.seen, &body.owner_id) {
        Ok(movie) => HttpResponse::Ok().json(movie),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}
