use crate::services::movie::{get_movies, Error};

use super::common::accordion::Accordion;
use super::common::loading::Loading;
use super::movie_item::MovieItem;
use yew::{function_component, html, use_effect_with_deps, Html};
use yew_hooks::use_async;

#[function_component(ListView)]
pub fn list_view() -> Html {
    let get_movies = use_async(async move { get_movies().await });

    {
        let get_movies_clone = get_movies.clone();
        use_effect_with_deps(
            move |_| {
                get_movies_clone.run();
                || ()
            },
            (),
        )
    }

    html! {
      <section class="[ list-view ] [ max-750 ]">
        <h1>{"Movie List"}</h1>
        // HANDLE LOADING
        {
          if get_movies.loading {
            html!{<Loading message={"Loading Movies!"}/>}
          } else {
            html!{}
          }
        }

        // HANDLE DATA
        {
          if let Some(movies) = &get_movies.data {
            html!{
              <Accordion title={"NACHO"}>
                <div class="[ list-view ]">
                  <ul>{
                    movies.into_iter().map(|movie| { html!{<li>
                      <MovieItem movie={movie.clone()} />
                    </li>}}).collect::<Html>()
                  }</ul>
                </div>
              </Accordion>
            }
          } else {
            html!{}
          }
        }

        // HANDLE ERROR
        {
          if let Some(error) = &get_movies.error {
            match error {
              Error::RequestError => html! { "RequestError" },
            }
          } else {
            html! {}
          }
        }
      </section>
    }
}
