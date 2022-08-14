use wasm_bindgen::JsCast;
use yew::{function_component, Html, html, use_state, Properties, Callback, events::{FocusEvent, InputEvent}};
use web_sys::{EventTarget, HtmlInputElement};

#[derive(Clone, PartialEq)]
struct Model {
    id: usize,
    text: String,
    done: bool
}

#[derive(Properties, PartialEq)]
struct ItemProps {
    todo: Model,
    // on_click: Callback<Video>
}

fn is_done(done: bool) -> String {
    return match done {
        true => "yes".to_string(),
        false => "no".to_string()
    }
}

#[function_component(Todo)]
fn todo(ItemProps { todo }: &ItemProps) -> Html {
    html! {
        <li data-qa="todo">
            <p>{format!("{}: {}", todo.id, todo.text)}</p>
            <p>{format!("Done: {}", is_done(todo.done))}</p>
            <button>{"Remove"}</button>
        </li>
    }
}

fn get_initial_state() -> Vec<Model> {
    return vec![
        Model {
            id: 1,
            text: "Adam".to_string(),
            done: false
        },
        Model {
            id: 2,
            text: "Maria".to_string(),
            done: true
        },
        Model {
            id: 3,
            text: "Imogen".to_string(),
            done: false
        },
    ];
}

#[function_component(Todos)]
fn todos() -> Html {
    let todos = use_state(get_initial_state);

    html! {
        <ul data-qa="todos">
            {todos.iter().map(|todo| html! {
                <Todo todo={todo.clone()} />
            }).collect::<Html>()}
        </ul>
    }
}

#[function_component(Form)]
fn form() -> Html {
    let todo = use_state(|| "".to_string());

    let on_submit = {
        Callback::from(move |event: FocusEvent| {
            let target: Option<EventTarget> = event.target();
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
        <form onsubmit={on_submit}>
            <input type="text" value={(*todo).clone()} oninput={on_input} />
            <button disabled={(*todo).clone() == ""}>{"Add"}</button>
        </form>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <section data-qa="app">
            <h1>{"Todos"}</h1>
            <Form />
            <Todos />
        </section>
    }
}

pub fn main() {
    yew::start_app::<App>();
}