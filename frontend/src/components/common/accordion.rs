use yew::{function_component, html, Children, Properties};

#[derive(Properties, PartialEq)]
pub struct AccordionProps {
    pub title: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Accordion)]
pub fn accordion(AccordionProps { children, title }: &AccordionProps) -> Html {
    html! {
        <div class="accordion">
            <div class="accordion-head">
                {title}
                <span class="material-icons">{"face"}</span>
            </div>
            <div class="accordion-body">
                { children.clone() }
            </div>
        </div>
    }
}
