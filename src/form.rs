use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{function_component, html, use_state, Properties, Callback, events::{FocusEvent, InputEvent}};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_add: Callback<String>
}

#[function_component(Form)]
pub fn form(Props { on_add }: &Props) -> Html {
    let todo = use_state(|| "".to_string());

    let on_add = {
        let todo = todo.clone();
        let on_add = on_add.clone();

        Callback::from(move |event: FocusEvent| {
            event.prevent_default();
            on_add.emit(todo.to_string());
            todo.set("".to_string());
        })
    };

    let on_input = {
        let todo = todo.clone();

        Callback::from(move |event: InputEvent| {
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|target| target.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| todo.set(input.value()));
        })
    };
    
    html! {
        <form onsubmit={on_add}>
            <input type="text" value={(*todo).clone()} oninput={on_input} />
            <button disabled={(*todo).clone() == ""}>{"Add"}</button>
        </form>
    }
}