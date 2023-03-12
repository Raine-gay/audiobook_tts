use pyo3::{prelude::*, types::PyDict};
use std::borrow::Borrow;

pub struct Synthesizer {
    locals: Py<PyDict>,
}

impl Synthesizer {
    /// Initialize a new synthesizer.
    /// This is NOT cheap to call, expect multiple seconds of delay.
    /// It is advised to make one synthesizer and keep it for the duration of your program.
    pub fn new(model_name: &str) -> Result<Self, PyErr> {
        Python::with_gil(|py| {
            let locals = PyDict::new(py);
            locals.set_item("model_name", model_name).unwrap();
            let locals: Py<PyDict> = locals.into();

            match py.run(
                r#"
import os
import sys
sys.stdout = open(os.devnull, 'w')
# Send the stdout to the void.

from TTS.api import TTS
tts = TTS(model_name=model_name, progress_bar=False, gpu=True)
                "#,
                None,
                Some(locals.as_ref(py).borrow()),
            ) {
                Ok(_) => Ok(Self { locals }),
                Err(error) => Err(error),
            }
        })
    }

    /// Synthesize a string into a wav file.
    pub fn generate(&mut self, text_chunk: &str, wav_path: &str) -> Result<(), PyErr> {
        Python::with_gil(|py| {
            let locals = self.locals.as_ref(py);
            locals.set_item("text_chunk", &text_chunk).unwrap();
            locals.set_item("wav_path", wav_path).unwrap();

            py.run(
                r#"tts.tts_to_file(text=text_chunk, file_path=wav_path)"#,
                None,
                Some(locals),
            )
        })
    }
}
