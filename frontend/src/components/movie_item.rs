use common::models::movie::Movie;
use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct MovieItemProps {
    pub movie: Movie,
}

#[function_component(MovieItem)]
pub fn accordion(MovieItemProps { movie }: &MovieItemProps) -> Html {
    html! {
        <div class="accordion">
            <div class="accordion-head">
                {movie.name}
            </div>
        </div>
    }
}
