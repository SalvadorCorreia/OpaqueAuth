use wasm_bindgen::prelude::*;

mod sanitizer;

#[wasm_bindgen]
pub fn evaluate_password(input: &str) -> String {
    let semantic_root = sanitizer::strip_padding(input);

    // Temporary return format for WebAssembly binding verification
    format!("Evaluated root: {}", semantic_root)
}
