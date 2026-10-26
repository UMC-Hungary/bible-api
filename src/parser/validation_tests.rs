#[cfg(test)]
mod validation_tests {
    use super::*;
    use crate::parser::validation::is_reference_pattern;

    #[test]
    fn test_is_reference_pattern_exact_book_match() {
        assert!(is_reference_pattern("gen"));
        assert!(is_reference_pattern("rom"));
        assert!(is_reference_pattern("mt"));
        assert!(is_reference_pattern("1cor"));
        assert!(is_reference_pattern("2tim"));
    }

    #[test]
    fn test_is_reference_pattern_with_separators() {
        assert!(is_reference_pattern("gen 1"));
        assert!(is_reference_pattern("gen:1"));
        assert!(is_reference_pattern("gen,1"));
        assert!(is_reference_pattern("mt 5:1"));
        assert!(is_reference_pattern("rom 12"));
    }

    #[test]
    fn test_is_reference_pattern_with_chapter_verse() {
        assert!(is_reference_pattern("gen 1:1"));
        assert!(is_reference_pattern("gen 1,1"));
        assert!(is_reference_pattern("gen 1.1"));
        assert!(is_reference_pattern("mt 5:1-2"));
    }

    #[test]
    fn test_is_reference_pattern_multiple_references() {
        assert!(is_reference_pattern("gen 1:1;2:3"));
        assert!(is_reference_pattern("mt 5:1-2;6:3-4"));
    }

    #[test]
    fn test_is_reference_pattern_with_translation() {
        assert!(is_reference_pattern("gen 1:1/KG"));
        assert!(is_reference_pattern("mt 5:1-2/ESV"));
    }

    #[test]
    fn test_is_reference_pattern_direct_chapter() {
        assert!(is_reference_pattern("gen1"));
        assert!(is_reference_pattern("mt5"));
        assert!(is_reference_pattern("rom12"));
    }

    #[test]
    fn test_is_reference_pattern_invalid_cases() {
        assert!(!is_reference_pattern(""));
        assert!(!is_reference_pattern("hello"));
        assert!(!is_reference_pattern("world"));
        assert!(!is_reference_pattern("123"));
        assert!(!is_reference_pattern("genesis")); // full name not in lowercase
    }

    #[test]
    fn test_is_reference_pattern_edge_cases() {
        assert!(is_reference_pattern("  gen 1  ")); // with spaces
        assert!(is_reference_pattern("GEN 1")); // uppercase
        assert!(!is_reference_pattern("gen0")); // invalid chapter
        assert!(!is_reference_pattern("gen100")); // chapter too high
    }

    #[test]
    fn test_is_reference_pattern_complex_patterns() {
        assert!(is_reference_pattern("1cor 15:1-4"));
        assert!(is_reference_pattern("2tim 3:16"));
        assert!(is_reference_pattern("gen 1:1-2,3-4"));
    }
}
