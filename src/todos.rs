use yew::{function_component, Html, html, Properties, Callback};

use crate::todo::Todo;
use crate::utils::Model;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub todos: Vec<Model>,
    pub on_remove: Callback<Model>
}

#[function_component(Todos)]
pub fn todos(Props { todos, on_remove }: &Props) -> Html {
    html! {
        <ul data-qa="todos">
            {todos.iter().map(|todo| html! {
                <Todo todo={todo.clone()} on_remove={on_remove} />
            }).collect::<Html>()}
        </ul>
    }
}
