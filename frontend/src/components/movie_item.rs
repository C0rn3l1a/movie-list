use common::models::movie::Movie;
use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct MovieItemProps {
    pub movie: Movie,
}

#[function_component(MovieItem)]
pub fn movie_item(MovieItemProps { movie }: &MovieItemProps) -> Html {
    let name = movie.name.clone();
    let seen = movie.seen;

    html! {
        <div class="movie-item">
            <input type="checkbox" checked={seen} />
            {name}
        </div>
    }
}
