use common::models::movie::Movie;
use yew::{function_component, html, use_state, Callback, Properties};

use crate::services::movie::update_movie;

#[derive(Properties, PartialEq)]
pub struct MovieItemProps {
    pub movie: Movie,
}

#[function_component(MovieItem)]
pub fn movie_item(MovieItemProps { movie }: &MovieItemProps) -> Html {
    let name = movie.name.clone();
    let seen = use_state(|| movie.seen);
    let movies = use_state(|| vec![]);

    let onclick = {
        let seen_clone = seen.clone();
        let movies_clone = movies.clone();

        Callback::from(|_| {
            //TODO make this work
            wasm_bindgen_futures::spawn_local(async move {
                let movies_clone = movies_clone.clone();
                let movies = update_movie().await;
                movies_clone.set(movies);

                seen_clone.set(!*seen_clone)
            });
        })
    };

    html! {
        <div class="movie-item">
            <input type="checkbox" checked={*seen} onclick={onclick}/>
            {name} {" - "} {movies.len()}
        </div>
    }
}
