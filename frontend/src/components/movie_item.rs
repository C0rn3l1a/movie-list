use common::models::movie::Movie;
use yew::{function_component, html, use_state, Callback, Html, Properties};
use yew_hooks::use_async;

use crate::services::movie::{update_movie, Error};

#[derive(Properties, PartialEq)]
pub struct MovieItemProps {
    pub movie: Movie,
}

#[function_component(MovieItem)]
pub fn movie_item(MovieItemProps { movie }: &MovieItemProps) -> Html {
    let name = movie.name.clone();
    let seen = use_state(|| movie.seen);

    let state = use_async(async move { update_movie().await });

    let onclick = {
        let state_clone = state.clone();
        Callback::from(move |_| {
            // You can trigger to run in callback or use_effect.
            state_clone.run()
        })
    };

    html! {
        <div class="movie-item">
            // HANDLE LOADING
            {
                if state.loading {
                    html!{ "Loading, wait a sec..." }
                } else {
                    html! {}
                }
            }
            // HANDLE DATA
            {
                if let Some(movs) = &state.data {
                    html!{
                        movs.into_iter().map(|mov| {
                            html!{<div key={mov.name.clone()}>{ format!("{}!!",mov.name) }</div>}
                        }).collect::<Html>()
                    }
                } else {
                    html! {}
                }
            }
            // HANDLE
            {
                if let Some(error) = &state.error {
                    match error {
                        Error::RequestError => html! { "RequestError" },
                    }
                }else {
                    html! {}
                }
            }

            <input type="checkbox" checked={*seen} onclick={onclick}/>
            {name} {" - "}
        </div>
    }
}
