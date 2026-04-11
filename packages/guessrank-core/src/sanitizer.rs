pub fn strip_padding(input: &str) -> &str {
    let stripped = input.trim_matches(|c: char| !c.is_alphabetic());

    // TODO: The length check below is a temporary quick-fix.
    // It prevents high-entropy random strings (e.g., "9!8@7#a5%") from being
    // destructively reduced to a single character ("a"). However, a hardcoded
    // length threshold is not a mathematically sound approach for differentiating
    // padded dictionary words from true randomness.
    // This requires deeper research (e.g., localized entropy checks) to fix properly.
    if stripped.len() < 3 { input } else { stripped }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strips_pure_padding_on_both_edges() {
        assert_eq!(strip_padding("!!password??"), "password");
        assert_eq!(strip_padding("123hello!"), "hello");
    }

    #[test]
    fn test_strips_pure_padding_on_single_edge() {
        assert_eq!(strip_padding("password??"), "password");
        assert_eq!(strip_padding("123hello"), "hello");
    }

    #[test]
    fn test_preserves_internal_characters_and_leetspeak() {
        assert_eq!(strip_padding("!!pass123word??"), "pass123word");
        assert_eq!(strip_padding("!123pass123word123?"), "pass123word");
        assert_eq!(strip_padding("!P@ssw0rd!"), "P@ssw0rd");
        assert_eq!(strip_padding("---Cloudflare2026---"), "Cloudflare");
    }

    #[test]
    fn test_handles_no_alphabetic_root() {
        assert_eq!(strip_padding("123456!"), "123456!");
        assert_eq!(strip_padding("!@#$%"), "!@#$%");
    }

    #[test]
    fn test_preserves_random_csprng_strings() {
        assert_eq!(strip_padding("9!8@7#6$a5%4^3&2"), "9!8@7#6$a5%4^3&2");
        assert_eq!(strip_padding("!@A#$"), "!@A#$");
        assert_eq!(strip_padding("12hi34"), "12hi34");
    }

    #[test]
    fn test_leaves_clean_string_alone() {
        assert_eq!(strip_padding("opaque"), "opaque");
        assert_eq!(strip_padding(""), "");
    }
}
