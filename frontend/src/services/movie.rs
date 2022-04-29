use common::models::movie::Movie;
use reqwasm::http::Request;

pub async fn update_movie() -> Vec<Movie> {
    let fetched_videos: Vec<Movie> = Request::get("https://yew.rs/tutorial/data.json")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    fetched_videos
}
