use base64::decode;
use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;
use image::load_from_memory;

// Import the `window.alert` function from the Web.
#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) {
    log(&"Grayscale Called".into());

    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"Image decoded".into());

    let img = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded".into());
}
