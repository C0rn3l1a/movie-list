use yew::{function_component, html, use_state, Callback, Children, Properties};

#[derive(Properties, PartialEq)]
pub struct AccordionProps {
    pub title: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Accordion)]
pub fn accordion(AccordionProps { children, title }: &AccordionProps) -> Html {
    let show = use_state(|| false);
    let onclick = {
        let show_clone = show.clone();
        Callback::from(move |_| show_clone.set(!*show_clone))
    };

    html! {
        <div class="accordion">
            <div class="accordion-head" {onclick}>
                {title}
                <span class="material-icons">if *show {{"keyboard_arrow_up"}} else {{"keyboard_arrow_down"}}</span>
            </div>
            if *show {
                <div class="accordion-body">
                    { children.clone() }
                </div>
            }
        </div>
    }
}
