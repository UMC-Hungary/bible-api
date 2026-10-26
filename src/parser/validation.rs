use super::books::get_books;

// Check if the input looks like a reference pattern
pub fn is_reference_pattern(term: &str) -> bool {
    // Look for patterns that suggest references:
    // - Numbers followed by book names (1Cor, 2Tim, etc.)
    // - Book names followed by numbers and proper separators (space, comma, colon)
    // - Contains semicolons (multiple references)
    // - Contains translation suffixes
    // - Simple book abbreviations or names (exact matches only)

    let term_trimmed = term.trim();
    let term_lower = term_trimmed.to_lowercase();

    // Check for common reference indicators
    let has_chapter_verse_separators =
        term.contains(',') || term.contains(':') || term.contains('.');
    let has_multiple_refs = term.contains(';');
    let has_translation_suffix = term.contains('/');
    let starts_with_number = term_trimmed
        .chars()
        .next()
        .is_some_and(|c| c.is_ascii_digit());

    // If it has structural indicators, it's likely valid
    if has_chapter_verse_separators || has_multiple_refs || has_translation_suffix {
        return true;
    }

    // books with their actual chapter counts
    let books = get_books();

    // Check for exact book name matches or book names with numbers
    for (book_names, max_chapters) in &books {
        for &book in book_names {
            if term_lower == book {
                return true; // Exact match like "gen", "rom", "mt", etc.
            }
            if starts_with_number
                && term_lower[1..].starts_with(book)
                && term_lower.len() == book.len() + 1
            {
                return true; // Patterns like "1kor", "2tim" (exactly book + 1 digit)
            }
            // Allow book names followed by separators OR valid chapter numbers
            if term_lower.starts_with(book) && term_trimmed.len() > book.len() {
                let after_book = &term_trimmed[book.len()..];
                // Allow proper separators (space, colon, comma)
                if after_book.starts_with(' ')
                    || after_book.starts_with(':')
                    || after_book.starts_with(',')
                {
                    return true; // "gen 1", "mt 5", "rom:3", etc.
                }
                // Allow direct numbers but only valid chapter numbers for this book
                if after_book.chars().all(|c| c.is_ascii_digit()) {
                    if let Ok(chapter) = after_book.parse::<u32>() {
                        if chapter >= 1 && chapter <= *max_chapters {
                            return true; // "mt5", "gen1", "rom12", etc.
                        }
                    }
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(!is_reference_pattern("invalidbook"));
        assert!(!is_reference_pattern("gen123"));
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
