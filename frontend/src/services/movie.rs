use common::models::movie::Movie;
use reqwasm::http::Request;
use serde::Deserialize;

// You can use thiserror to define your errors.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub enum Error {
    RequestError,
}

pub async fn update_movie() -> Result<Vec<Movie>, Error> {
    let response = Request::get("http://localhost:3000/movies")
        .send()
        .await
        .unwrap()
        .json::<Vec<Movie>>()
        .await;

    match response {
        Ok(movies) => Ok(movies),
        Err(error) => {
            log::debug!("Error: {:?}", error.to_string());
            Err(Error::RequestError)
        }
    }
}

pub async fn get_movies() -> Result<Vec<Movie>, Error> {
    let response = Request::get("http://localhost:3000/movies")
        .send()
        .await
        .unwrap()
        .json::<Vec<Movie>>()
        .await;

    match response {
        Ok(movies) => Ok(movies),
        Err(error) => {
            log::debug!("Error: {:?}", error.to_string());
            Err(Error::RequestError)
        }
    }
}
