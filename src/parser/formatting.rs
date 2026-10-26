use super::books::get_hungarian_book_name;
use crate::types::ParsedReference;

// Extract translation suffix like "/KG" or "/*"
pub fn extract_translation_suffix(term: &str) -> (&str, Option<&str>) {
    if let Some(slash_pos) = term.rfind('/') {
        let main_part = &term[..slash_pos];
        let suffix = &term[slash_pos + 1..];
        if suffix == "*" || suffix.len() <= 3 {
            // Handle /* and short translation codes
            return (main_part, Some(suffix));
        }
    }
    (term, None)
}

// Format a parsed reference back to a normalized label
pub fn format_parsed_reference_label(parsed_ref: &ParsedReference) -> String {
    let mut label = parsed_ref.book.clone();

    if let Some(chapter_from) = parsed_ref.chapter_from {
        if let Some(chapter_to) = parsed_ref.chapter_to {
            if chapter_from == chapter_to {
                label.push_str(&format!(" {chapter_from}"));
            } else {
                label.push_str(&format!(" {chapter_from}-{chapter_to}"));
            }
        } else {
            label.push_str(&format!(" {chapter_from}"));
        }

        if let Some(verse_from) = parsed_ref.verse_from {
            if let Some(verse_to) = parsed_ref.verse_to {
                if verse_from == verse_to {
                    label.push_str(&format!(":{verse_from}"));
                } else {
                    label.push_str(&format!(":{verse_from}-{verse_to}"));
                }
            } else {
                label.push_str(&format!(":{verse_from}"));
            }
        }
    }

    label
}

// Format a parsed reference with Hungarian book name
pub fn format_hungarian_reference_label(parsed_ref: &ParsedReference) -> String {
    let english_label = format_parsed_reference_label(parsed_ref);

    // Replace English book name with Hungarian
    if let Some(hungarian_book) = get_hungarian_book_name(parsed_ref.book_id) {
        english_label.replacen(&parsed_ref.book, &hungarian_book, 1)
    } else {
        english_label
    }
}
