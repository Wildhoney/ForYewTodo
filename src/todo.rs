use yew::{function_component, html, Properties, Callback};

use crate::utils::Model;

pub fn is_done(done: bool) -> String {
    return match done {
        true => "yes".to_string(),
        false => "no".to_string()
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub todo: Model,
    pub on_remove: Callback<Model>
}

#[function_component(Todo)]
pub fn todo(Props { todo, on_remove }: &Props) -> Html {
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