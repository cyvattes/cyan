use serde::{Serialize};

#[derive(Serialize)]
pub struct NGram {
    n_1: Vec<String>,
    n_2: Vec<String>,
    n_3: Vec<String>,
    n_4: Vec<String>,
}

impl NGram {
    pub fn new() -> NGram {
        NGram {
            n_1: vec![String::new()],
            n_2: vec![String::new()],
            n_3: vec![String::new()],
            n_4: vec![String::new()],
        }
    }
}

