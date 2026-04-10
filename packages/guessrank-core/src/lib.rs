use wasm_bindgen::prelude::*;

// This is a simple stub to ensure the compilation bridge works.
#[wasm_bindgen]
pub fn evaluate_password(input: &str) -> String {
    format!("Evaluated: {}", input)
}

// TODO: Implement actual padding removal logic
pub fn strip_padding(input: &str) -> &str {
    input // Dummy implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_removes_symbols() {
        assert_eq!(strip_padding("!!password??"), "password");
    }

    #[test]
    fn test_removes_numbers_and_symbols() {
        assert_eq!(strip_padding("123hello!"), "hello");
    }

    #[test]
    fn test_leaves_clean_string_alone() {
        assert_eq!(strip_padding("cloudflare"), "cloudflare");
    }
}
