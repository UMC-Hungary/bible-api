use super::books::{get_books, is_valid_book, normalize_book_name};
use super::formatting::{
    extract_translation_suffix, format_hungarian_reference_label, format_parsed_reference_label,
};
use super::validation::is_reference_pattern;
use crate::db::get_book_id_from_name;
use crate::types::{ParsedReference, SimplifiedResponse};
use regex::Regex;

// Parse a reference into structured components
pub fn parse_reference(reference: &str) -> Option<ParsedReference> {
    let (clean_ref, _translation) = extract_translation_suffix(reference);

    // Normalize comma format to colon format (mt4,1-2 -> mt4:1-2)
    let normalized_ref = clean_ref.replace(',', ":");

    // First try to split book name from chapter/verse using known book patterns
    if let Some((book, chapter_verse_part)) = split_book_from_reference(&normalized_ref) {
        if !is_valid_book(&book) {
            return None;
        }

        // Normalize book name to canonical short form
        let normalized_book = normalize_book_name(&book)?;

        if chapter_verse_part.is_empty() {
            // Just a book name
            let book_id = get_book_id_from_name(&normalized_book);
            return Some(ParsedReference {
                book: normalized_book,
                book_id,
                chapter_from: None,
                chapter_to: None,
                verse_from: None,
                verse_to: None,
            });
        }

        // Parse chapter:verse part
        let chapter_verse_regex =
            Regex::new(r"^(\d+)(?::(\d+)(?:-(\d+))?)?(?:-(\d+)(?::(\d+))?)?$").ok()?;

        if let Some(caps) = chapter_verse_regex.captures(chapter_verse_part.trim()) {
            let chapter_from = caps.get(1)?.as_str().parse::<u32>().ok()?;
            let verse_from = caps.get(2).and_then(|m| m.as_str().parse::<u32>().ok());
            let verse_to = caps.get(3).and_then(|m| m.as_str().parse::<u32>().ok());
            let chapter_to = caps.get(4).and_then(|m| m.as_str().parse::<u32>().ok());
            let verse_to_end = caps.get(5).and_then(|m| m.as_str().parse::<u32>().ok());

            let book_id = get_book_id_from_name(&normalized_book);
            return Some(ParsedReference {
                book: normalized_book,
                book_id,
                chapter_from: Some(chapter_from),
                chapter_to,
                verse_from,
                verse_to: verse_to.or(verse_to_end),
            });
        }
    }

    // Fallback: try original space-separated format
    let reference_regex =
        Regex::new(r"(?i)^(.+?)\s+(\d+)(?::(\d+)(?:-(\d+))?)?(?:-(\d+)(?::(\d+))?)?$").ok()?;

    if let Some(caps) = reference_regex.captures(normalized_ref.trim()) {
        let book = caps.get(1)?.as_str().trim().to_string();

        // Validate book name
        if !is_valid_book(&book) {
            return None;
        }

        // Normalize book name to canonical short form
        let normalized_book = normalize_book_name(&book)?;

        let chapter_from = caps.get(2)?.as_str().parse::<u32>().ok()?;
        let verse_from = caps.get(3).and_then(|m| m.as_str().parse::<u32>().ok());
        let verse_to = caps.get(4).and_then(|m| m.as_str().parse::<u32>().ok());
        let chapter_to = caps.get(5).and_then(|m| m.as_str().parse::<u32>().ok());
        let verse_to_end = caps.get(6).and_then(|m| m.as_str().parse::<u32>().ok());

        let book_id = get_book_id_from_name(&normalized_book);
        Some(ParsedReference {
            book: normalized_book,
            book_id,
            chapter_from: Some(chapter_from),
            chapter_to,
            verse_from,
            verse_to: verse_to.or(verse_to_end),
        })
    } else {
        None
    }
}

// Helper function to split book name from chapter/verse part
fn split_book_from_reference(reference: &str) -> Option<(String, String)> {
    let books = get_books();
    let ref_lower = reference.to_lowercase();

    // Check each book pattern to see if the reference starts with it
    for (book_names, _) in &books {
        for &book in book_names {
            if ref_lower.starts_with(book) {
                let after_book = &reference[book.len()..];

                // If there's content after the book name, split it
                if !after_book.is_empty() {
                    // Check if the content after book is numeric or contains chapter/verse patterns
                    if after_book
                        .chars()
                        .all(|c| c.is_ascii_digit() || c == ':' || c == '-')
                    {
                        // Return the original book name from the input, preserving case
                        let original_book = &reference[0..book.len()];
                        return Some((original_book.to_string(), after_book.to_string()));
                    }
                } else {
                    // Just the book name - return the original case
                    let original_book = &reference[0..book.len()];
                    return Some((original_book.to_string(), String::new()));
                }
            }
        }
    }

    None
}

// Parse terms and extract references with contextual parsing
fn parse_terms_for_references(input: &str) -> Vec<SimplifiedResponse> {
    let mut current_book: Option<String> = None;
    let mut current_chapter: Option<u32> = None;

    input
        .split(';')
        .map(|part| part.trim())
        .filter(|part| !part.is_empty())
        .filter_map(|part| {
            // Try direct parsing first
            if let Some(parsed) = parse_reference(part) {
                // Update context for subsequent references
                current_book = Some(parsed.book.clone());
                current_chapter = parsed.chapter_from;
                Some(parsed)
            } else if let Some(ref book) = current_book {
                // Fallback to contextual parsing
                if let Some(contextual_ref) = parse_contextual_part(part, book, current_chapter) {
                    current_chapter = contextual_ref.chapter_from;
                    Some(contextual_ref)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .map(|parsed| {
            let label = format_parsed_reference_label(&parsed);
            let hungarian_label = format_hungarian_reference_label(&parsed);
            let link = format!("/{label}");
            SimplifiedResponse {
                label,
                hungarian_label,
                link,
                parsed_refs: vec![parsed],
                verses: Vec::new(),
                verses_as_text: Vec::new(),
            }
        })
        .collect()
}

// Parse a partial reference in the context of a current book and optionally a chapter
fn parse_contextual_part(
    part: &str,
    current_book: &str,
    current_chapter: Option<u32>,
) -> Option<ParsedReference> {
    // Normalize comma format to colon format
    let normalized_part = part.replace(',', ":");

    // Normalize the current book name to canonical short form
    let normalized_book = normalize_book_name(current_book)?;

    // Check if this is chapter:verse format (like "3:1-2")
    if let Some(colon_pos) = normalized_part.find(':') {
        let chapter_part = &normalized_part[..colon_pos];
        let verse_part = &normalized_part[colon_pos + 1..];

        if let Ok(chapter) = chapter_part.parse::<u32>() {
            let (verse_from, verse_to) = parse_verse_range(verse_part);
            let book_id = get_book_id_from_name(&normalized_book);
            return Some(ParsedReference {
                book: normalized_book,
                book_id,
                chapter_from: Some(chapter),
                chapter_to: None, // Keep null for single chapter references to match main parser
                verse_from,
                verse_to,
            });
        }
    }

    // Check if this is just verse numbers (like "1-2" in same chapter context)
    if normalized_part
        .chars()
        .all(|c| c.is_ascii_digit() || c == '-')
    {
        // Use inherited chapter context if available
        if let Some(chapter) = current_chapter {
            let (verse_from, verse_to) = parse_verse_range(&normalized_part);
            let book_id = get_book_id_from_name(&normalized_book);
            return Some(ParsedReference {
                book: normalized_book,
                book_id,
                chapter_from: Some(chapter),
                chapter_to: None,
                verse_from,
                verse_to,
            });
        }
        // If no chapter context, we can't parse verse-only patterns
        return None;
    }

    // Check if this is just a chapter number
    if let Ok(chapter) = normalized_part.parse::<u32>() {
        let book_id = get_book_id_from_name(&normalized_book);
        return Some(ParsedReference {
            book: normalized_book,
            book_id,
            chapter_from: Some(chapter),
            chapter_to: None, // Keep null for single chapter references to match main parser
            verse_from: None,
            verse_to: None,
        });
    }

    None
}

// Parse verse ranges that can include ranges like "1-2"
fn parse_verse_range(verse_str: &str) -> (Option<u32>, Option<u32>) {
    if let Some(dash_pos) = verse_str.find('-') {
        // Range: "1-2"
        let from_str = &verse_str[..dash_pos];
        let to_str = &verse_str[dash_pos + 1..];

        let from = from_str.parse::<u32>().ok();
        let to = to_str.parse::<u32>().ok();
        (from, to)
    } else {
        // Single number: "1"
        let num = verse_str.parse::<u32>().ok();
        (num, num)
    }
}

// Smart parsing that handles multiple reference formats
pub fn smart_parse_reference(input: &str) -> Vec<SimplifiedResponse> {
    let mut all_refs = Vec::new();

    // Try parsing multiple references separated by semicolons first
    let semicolon_refs = parse_terms_for_references(input);
    all_refs.extend(semicolon_refs);

    // Try parsing with different delimiters (but avoid comma since it's used in verse format)
    let alternative_splits = vec![
        input.split(" and ").collect::<Vec<_>>(),
        input.split(" & ").collect::<Vec<_>>(),
    ];

    for splits in alternative_splits {
        if splits.len() > 1 {
            // Only process if there are multiple parts
            for part in splits {
                let part = part.trim();
                if is_reference_pattern(part) {
                    if let Some(parsed) = parse_reference(part) {
                        let label = format_parsed_reference_label(&parsed);
                        let hungarian_label = format_hungarian_reference_label(&parsed);
                        let link = format!("/{label}");
                        let simple_ref = SimplifiedResponse {
                            label,
                            hungarian_label,
                            link,
                            parsed_refs: vec![parsed],
                            verses: Vec::new(),
                            verses_as_text: Vec::new(),
                        };

                        // Check if we already have this reference
                        if !all_refs.iter().any(|r| r.label == simple_ref.label) {
                            all_refs.push(simple_ref);
                        }
                    }
                }
            }
        }
    }

    // If no results yet, try parsing as a single reference
    if all_refs.is_empty() && is_reference_pattern(input) {
        if let Some(parsed) = parse_reference(input) {
            let label = format_parsed_reference_label(&parsed);
            let hungarian_label = format_hungarian_reference_label(&parsed);
            let link = format!("/{label}");
            let simple_ref = SimplifiedResponse {
                label,
                hungarian_label,
                link,
                parsed_refs: vec![parsed],
                verses: Vec::new(),
                verses_as_text: Vec::new(),
            };
            all_refs.push(simple_ref);
        }
    }

    all_refs
}
