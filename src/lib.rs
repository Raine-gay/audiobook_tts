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
    ///
    /// Use the filter input bool if you wish for this crate to attempt to remove junk from the input.
    /// It is recommend to use this feature as junk in the input can cause the TTS generator to crash.
    /// Please note: That this feature hasn't been tested and so could have a lil bit of unintended behavior.
    pub fn generate(
        &mut self,
        string_input: &str,
        wav_path: &str,
        filter_input: bool,
    ) -> Result<(), PyErr> {
        let mut string_input = string_input.to_string();
        if filter_input {
            string_input = filter_string_input(string_input);
        }
        if !string_input.is_empty() {
            Python::with_gil(|py| {
                let locals = self.locals.as_ref(py);
                locals.set_item("text_chunk", &string_input).unwrap();
                locals.set_item("wav_path", wav_path).unwrap();

                py.run(
                    r#"tts.tts_to_file(text=text_chunk, file_path=wav_path)"#,
                    None,
                    Some(locals),
                )
            })?
        }

        Ok(())
    }
}

fn filter_string_input(string_input: String) -> String {
    let mut input_chars: Vec<char> = string_input.chars().collect();

    let mut contains_alphanumeric = false; // This checks
    for char in &input_chars {
        if char.is_ascii_alphanumeric() {
            contains_alphanumeric = true;
            break;
        }
    }
    if !contains_alphanumeric {
        return String::new();
    }

    const WHITELISTED_CHARS: [char; 48] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0',
        '$', '£', '!', '.', '?', ',', '\x27', '&', ';', ':', ' ', '-'
    ];
    for i in 0..input_chars.len() {
        if !WHITELISTED_CHARS.contains(&input_chars[i].to_ascii_lowercase()) {
            input_chars[i] = '\x00';
        }
    }

    let string_input: String = input_chars.into_iter().collect();
    string_input.replace('\x00', "")
}
