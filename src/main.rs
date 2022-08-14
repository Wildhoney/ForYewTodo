use wasm_bindgen::JsCast;
use yew::{function_component, Html, html, use_state, Properties, Callback, events::{FocusEvent, InputEvent}};
use web_sys::{EventTarget, HtmlInputElement};

#[derive(Clone, PartialEq)]
struct Model {
    id: usize,
    text: String,
    done: bool
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

fn is_done(done: bool) -> String {
    return match done {
        true => "yes".to_string(),
        false => "no".to_string()
    }
}

#[derive(Properties, PartialEq)]
struct ItemProps {
    todo: Model,
    on_remove: Callback<Model>
}

#[function_component(Todo)]
fn todo(ItemProps { todo, on_remove }: &ItemProps) -> Html {
    let on_remove = {
        let todo = todo.clone();
        let on_remove = on_remove.clone();

        Callback::from(move |_| {
            on_remove.emit(todo.clone())
        })
    };

    html! {
        <li data-qa="todo">
            <p>{format!("{}: {}", todo.id, todo.text)}</p>
            <p>{format!("Done: {}", is_done(todo.done))}</p>
            <button onclick={on_remove}>{"Remove"}</button>
        </li>
    }
}

#[derive(Properties, PartialEq)]
struct TodoProps {
    todos: Vec<Model>,
    on_remove: Callback<Model>
}

#[function_component(Todos)]
fn todos(TodoProps { todos, on_remove }: &TodoProps) -> Html {
    html! {
        <ul data-qa="todos">
            {todos.iter().map(|todo| html! {
                <Todo todo={todo.clone()} on_remove={on_remove} />
            }).collect::<Html>()}
        </ul>
    }
}

#[derive(Properties, PartialEq)]
struct FormProps {
    on_add: Callback<Model>
}

#[function_component(Form)]
fn form(FormProps { on_add }: &FormProps) -> Html {
    let todo = use_state(|| "".to_string());

    let on_add = {
        let todo = todo.clone();
        let on_add = on_add.clone();

        Callback::from(move |event: FocusEvent| {
            event.prevent_default();
            let new_todo = Model { id: 4, text: todo.to_string(), done: false };
            on_add.emit(new_todo);
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

#[function_component(App)]
fn app() -> Html {
    let todos = use_state(get_initial_state);

    let on_add = {
        let todos = todos.clone();

        Callback::from(move |model: Model| {
            let mut updated_todos: Vec<Model> = todos.to_vec().clone();
            updated_todos.push(model);
            todos.set(updated_todos);
        })
    };

    let on_remove = {
        let todos = todos.clone();

        Callback::from(move |model: Model| {
            let mut updated_todos: Vec<Model> = todos.to_vec().clone();
            let index = todos.iter().position(|x| *x == model).unwrap();
            updated_todos.remove(index);
            todos.set(updated_todos);
        })
    };

    html! {
        <section data-qa="app">
            <h1>{"Todos"}</h1>
            <Form on_add={on_add} />
            <Todos todos={(*todos).clone()} on_remove={on_remove.clone()} />
        </section>
    }
}

pub fn main() {
    yew::start_app::<App>();
}