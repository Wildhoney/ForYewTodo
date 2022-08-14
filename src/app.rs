use yew::{function_component, html, use_state, Callback};

use crate::form::Form;
use crate::todos::Todos;
use crate::utils::{Model, get_initial_state};

#[function_component(App)]
pub fn app() -> Html {
    let todos = use_state(get_initial_state);

    let on_add = {
        let todos = todos.clone();

        Callback::from(move |value: String| {
            let mut updated_todos: Vec<Model> = todos.to_vec().clone();
            let todo = Model { id: todos.len() + 1, text: value, done: false };
            updated_todos.push(todo);
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