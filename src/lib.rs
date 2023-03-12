use pyo3::{prelude::*, types::PyDict};

#[derive(Debug)]
pub struct Synthesizer {
    locals: Py<PyDict>,
}

impl Synthesizer {
    pub fn new(model_name: &str) -> Self {
        
    }
}