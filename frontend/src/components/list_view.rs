use super::common::accordion::Accordion;
use super::movie_item::MovieItem;
use common::models::movie::Movie;
use yew::{function_component, html};

#[function_component(ListView)]
pub fn list_view() -> Html {
    html! {
      <section class="[ list-view ] [ max-750 ]">
        <h1>{"Movie List"}</h1>
        <ul>
          <li>
            <Accordion title={"NACHO"}>
              <div class="[ list-view ]">
                <ul>
                  <li>
                    <MovieItem movie={Movie {
                      id: 0,
                      owner_id: 0,
                      name: String::from("Terminator 2"),
                      seen: false
                    }} />
                  </li>
                  <li>
                    <MovieItem movie={Movie {
                      id: 0,
                      owner_id: 0,
                      name: String::from("Terminator 2"),
                      seen: false
                    }} />
                  </li>
                  <li>
                    <MovieItem movie={Movie {
                      id: 0,
                      owner_id: 0,
                      name: String::from("Terminator 2"),
                      seen: false
                    }} />
                  </li>
                </ul>
              </div>
            </Accordion>
          </li>
          <li>

          </li>
        </ul>
      </section>
    }
}
