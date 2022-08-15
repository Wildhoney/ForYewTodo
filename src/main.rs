pub mod app;

pub mod components {
    pub mod form;
    pub mod todo;
    pub mod todos;
}

pub mod utils;

use yew;

pub fn main() {
    yew::start_app::<app::App>();
}