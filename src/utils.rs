#[derive(Clone, PartialEq)]
pub struct Model {
    pub id: usize,
    pub text: String,
    pub done: bool
}

pub fn get_initial_state() -> Vec<Model> {
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