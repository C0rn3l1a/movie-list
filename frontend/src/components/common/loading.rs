use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct LoadingProps {
    pub message: String,
}

#[function_component(Loading)]
pub fn loading_component(LoadingProps { message }: &LoadingProps) -> Html {
    html! {
        <div class="loading-spinner">
            <div class="sk-fading-circle">
                <div class="sk-circle1 sk-circle"></div>
                <div class="sk-circle2 sk-circle"></div>
                <div class="sk-circle3 sk-circle"></div>
                <div class="sk-circle4 sk-circle"></div>
                <div class="sk-circle5 sk-circle"></div>
                <div class="sk-circle6 sk-circle"></div>
                <div class="sk-circle7 sk-circle"></div>
                <div class="sk-circle8 sk-circle"></div>
                <div class="sk-circle9 sk-circle"></div>
                <div class="sk-circle10 sk-circle"></div>
                <div class="sk-circle11 sk-circle"></div>
                <div class="sk-circle12 sk-circle"></div>
            </div>
            <span class="spinner-text">{message}</span>
        </div>
    }
}
