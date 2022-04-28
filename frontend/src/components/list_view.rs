use super::common::accordion::Accordion;
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
                    <div>
                      {"PELI 3"}
                    </div>
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
