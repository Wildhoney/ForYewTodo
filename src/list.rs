use yew::{function_component, html};

#[function_component(List)]
pub fn List() -> Html {
    html! {
        <ul>
            <li>{"Adam"}</li>
            <li>{"Maria"}</li>
            <li>{"Imogen"}</li>
        </ul>
    }
}