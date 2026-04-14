use wasm_bindgen::prelude::*;

mod sanitizer;

/// Evaluates the guessability of `input` by stripping non-semantic padding and
/// returning the extracted semantic root. Currently returns a diagnostic string;
/// will return a numeric GuessRank score once scoring is implemented.
#[wasm_bindgen]
pub fn evaluate_password(input: &str) -> String {
    let semantic_root = sanitizer::strip_padding(input);

    // Placeholder until GuessRank scoring (scoring.rs) is implemented; enables
    // manual Wasm binding verification without returning a null value.
    format!("Evaluated root: {}", semantic_root)
}
