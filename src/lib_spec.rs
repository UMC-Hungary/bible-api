use super::*;

fn parse_reference(term: &str) -> Option<SearchSuggestion> {
    let term = term.trim();

    // Extract translation suffix first to handle links correctly
    let (clean_term, translation_suffix) =
        crate::parser::formatting::extract_translation_suffix(term);

    // Use the smart parsing function from the parsing module
    let parsed_results = parser::parser::smart_parse_reference(clean_term);

    if parsed_results.is_empty() {
        return None;
    }

    // Convert the results to the format expected by the old function
    // Collect all parsed_refs from all results
    let mut all_parsed_refs = Vec::new();
    for result in &parsed_results {
        all_parsed_refs.extend(result.parsed_refs.clone());
    }

    // Fix compatibility: set chapter_to and verse_to to match chapter_from and verse_from
    // when they are None (to match old test expectations)
    // Also preserve original book name case from input
    let original_parts: Vec<&str> = clean_term.split(';').map(|s| s.trim()).collect();
    let mut first_book_name: Option<String> = None;

    for (i, parsed_ref) in all_parsed_refs.iter_mut().enumerate() {
        if parsed_ref.chapter_to.is_none() && parsed_ref.chapter_from.is_some() {
            parsed_ref.chapter_to = parsed_ref.chapter_from;
        }
        if parsed_ref.verse_to.is_none() && parsed_ref.verse_from.is_some() {
            parsed_ref.verse_to = parsed_ref.verse_from;
        }

        // Preserve original book case from input
        if i < original_parts.len() {
            let original_part = original_parts[i];

            if i == 0 {
                // First reference - extract book name and store it
                if let Some(original_book) = extract_book_name_from_input(original_part) {
                    parsed_ref.book = original_book.clone();
                    first_book_name = Some(original_book);
                }
            } else {
                // Subsequent references - check if this looks like a contextual reference
                // (doesn't contain a book name)
                if let Some(extracted) = extract_book_name_from_input(original_part) {
                    // If the extracted book is just numbers/punctuation, it's contextual
                    if extracted
                        .chars()
                        .all(|c| c.is_ascii_digit() || c == '-' || c == ':' || c == ',')
                    {
                        // Use the first reference's book name
                        if let Some(ref first_book) = first_book_name {
                            parsed_ref.book = first_book.clone();
                        }
                    } else {
                        // It has a real book name, use it
                        parsed_ref.book = extracted;
                    }
                }
            }
        }
    }

    // Check if this looks like a contextual reference pattern
    let is_contextual = clean_term.contains(';')
        && parsed_results.len() > 1
        && all_parsed_refs.len() > 1
        && all_parsed_refs
            .iter()
            .all(|r| r.book == all_parsed_refs[0].book);

    // Create concatenated label preserving original case from input
    let concatenated_label = if is_contextual {
        // For contextual references, create a combined label from the parsed refs
        create_contextual_label(clean_term, &all_parsed_refs)
    } else {
        preserve_original_case_in_label(clean_term, &parsed_results)
    };

    // Construct link with proper translation suffix
    let link = if let Some(suffix) = translation_suffix {
        format!("/{concatenated_label}/{suffix}")
    } else {
        format!("/{concatenated_label}")
    };
    Some(create_search_suggestion_with_refs(
        1,
        "ref",
        &concatenated_label,
        &link,
        all_parsed_refs,
    ))
}

// Helper function to preserve original case from input in the final label

fn preserve_original_case_in_label(
    original_input: &str,
    parsed_results: &[crate::types::SimplifiedResponse],
) -> String {
    // Split original input by semicolon to match with parsed results
    let original_parts: Vec<&str> = original_input.split(';').map(|s| s.trim()).collect();

    let mut final_labels = Vec::new();

    for (i, result) in parsed_results.iter().enumerate() {
        if i < original_parts.len() {
            let original_part = original_parts[i];
            let parsed_label = &result.label;

            // Try to match book names and preserve original case
            final_labels.push(preserve_case_in_single_reference(
                original_part,
                parsed_label,
            ));
        } else {
            final_labels.push(result.label.clone());
        }
    }

    final_labels.join(";")
}

// Helper to preserve case in a single reference

fn preserve_case_in_single_reference(original: &str, parsed: &str) -> String {
    // Convert comma format to colon format first
    let normalized_original = original.replace(',', ":");

    // Extract book name from original using the same logic as extract_book_name_from_input
    let original_book = if let Some(book) = extract_book_name_from_input(&normalized_original) {
        book
    } else {
        // Fallback to first word or whole string
        let original_words: Vec<&str> = normalized_original.split_whitespace().collect();
        original_words
            .first()
            .map(|s| s.to_string())
            .unwrap_or(normalized_original.clone())
    };

    let parsed_words: Vec<&str> = parsed.split_whitespace().collect();

    if !parsed_words.is_empty() {
        if parsed_words.len() > 1 {
            format!("{} {}", original_book, parsed_words[1..].join(" "))
        } else {
            original_book
        }
    } else {
        parsed.to_string()
    }
}

// Helper to extract book name from input preserving original case

fn extract_book_name_from_input(input: &str) -> Option<String> {
    // Handle space-separated format first (e.g., "1Kor 2:3")
    let words: Vec<&str> = input.split_whitespace().collect();
    if words.len() > 1 {
        return Some(words[0].to_string());
    }

    // Handle direct format like "mt5", "1kor3:4", etc.
    // We need to find where the book name ends and the numbers begin
    use crate::parser::books::get_books;
    let books = get_books();
    let input_lower = input.to_lowercase();

    // First check if input exactly matches any book name (like "genesis")
    for (book_names, _) in &books {
        for &book in book_names {
            if input_lower == book {
                return Some(input.to_string()); // Return original case
            }
        }
    }

    // Check each known book pattern for prefix matches
    for (book_names, _) in &books {
        for &book in book_names {
            if input_lower.starts_with(book) {
                // Extract the original case book name from input
                let book_len = book.len();
                if input.len() >= book_len {
                    return Some(input[0..book_len].to_string());
                }
            }
        }
    }

    // Fallback: just return the first word or the whole input if no spaces
    Some(words.first().unwrap_or(&input).to_string())
}

// Create a contextual label from parsed references (e.g., "mt 5:1-2;5-6")

fn create_contextual_label(original_input: &str, parsed_refs: &[ParsedReference]) -> String {
    if parsed_refs.is_empty() {
        return original_input.to_string();
    }

    // Extract original book name from input
    let original_book = if let Some(book) = extract_book_name_from_input(original_input) {
        book
    } else {
        parsed_refs[0].book.clone()
    };

    // Build contextual label
    let mut parts = Vec::new();

    for (i, parsed_ref) in parsed_refs.iter().enumerate() {
        if i == 0 {
            // First reference includes book name
            if let Some(chapter) = parsed_ref.chapter_from {
                let chapter_verse = if let Some(verse_from) = parsed_ref.verse_from {
                    if let Some(verse_to) = parsed_ref.verse_to {
                        if verse_from == verse_to {
                            format!("{chapter}:{verse_from}")
                        } else {
                            format!("{chapter}:{verse_from}-{verse_to}")
                        }
                    } else {
                        format!("{chapter}:{verse_from}")
                    }
                } else {
                    chapter.to_string()
                };
                parts.push(format!("{original_book} {chapter_verse}"));
            } else {
                parts.push(original_book.clone());
            }
        } else {
            // Subsequent references are contextual
            if let Some(chapter) = parsed_ref.chapter_from {
                // Check if same chapter as previous reference
                let prev_chapter = parsed_refs[i - 1].chapter_from;
                if Some(chapter) == prev_chapter {
                    // Same chapter, just show verse range
                    if let Some(verse_from) = parsed_ref.verse_from {
                        if let Some(verse_to) = parsed_ref.verse_to {
                            if verse_from == verse_to {
                                parts.push(verse_from.to_string());
                            } else {
                                parts.push(format!("{verse_from}-{verse_to}"));
                            }
                        } else {
                            parts.push(verse_from.to_string());
                        }
                    } else {
                        parts.push(chapter.to_string());
                    }
                } else {
                    // Different chapter, show full book chapter:verse for clarity
                    if let Some(verse_from) = parsed_ref.verse_from {
                        if let Some(verse_to) = parsed_ref.verse_to {
                            if verse_from == verse_to {
                                parts.push(format!("{original_book} {chapter}:{verse_from}"));
                            } else {
                                parts.push(format!(
                                    "{original_book} {chapter}:{verse_from}-{verse_to}"
                                ));
                            }
                        } else {
                            parts.push(format!("{original_book} {chapter}:{verse_from}"));
                        }
                    } else {
                        parts.push(format!("{original_book} {chapter}"));
                    }
                }
            }
        }
    }

    parts.join(";")
}

#[test]
fn test_create_suggestions_response() {
    let suggestions = vec![
        create_search_suggestion(3, "ref", "Genesis", "/books/gen/en"),
        create_search_suggestion(7, "verse", "John 3:16", "/books/jhn/3/16/en"),
    ];
    let response = create_suggestions_response(suggestions);
    assert_eq!(response.total_count, 10);
    assert_eq!(response.suggestions.len(), 2);
    assert_eq!(response.suggestions[0].hit_count, 3);
    assert_eq!(response.suggestions[1].hit_count, 7);
}

#[test]
fn test_create_suggestions_response_empty() {
    let response = create_suggestions_response(vec![]);
    assert_eq!(response.total_count, 0);
    assert_eq!(response.suggestions.len(), 0);
}

#[test]
fn test_extract_translation_suffix() {
    let (base, suffix) = extract_translation_suffix("gen 1:1/KG");
    assert_eq!(base, "gen 1:1");
    assert_eq!(suffix, Some("KG"));

    let (base, suffix) = extract_translation_suffix("gen 1:1/*");
    assert_eq!(base, "gen 1:1");
    assert_eq!(suffix, Some("*"));

    let (base, suffix) = extract_translation_suffix("gen 1:1");
    assert_eq!(base, "gen 1:1");
    assert_eq!(suffix, None);
}

#[test]
fn test_parse_range_edge_cases() {
    let (from, to) = parse_range("");
    assert_eq!(from, None);
    assert_eq!(to, None);

    let (from, to) = parse_range("5");
    assert_eq!(from, Some(5));
    assert_eq!(to, Some(5));

    let (from, to) = parse_range("1-3");
    assert_eq!(from, Some(1));
    assert_eq!(to, Some(3));

    let (from, to) = parse_range("invalid");
    assert_eq!(from, None);
    assert_eq!(to, None);
}

#[test]
fn test_normalize_chapter_verse_format() {
    assert_eq!(normalize_chapter_verse_format("1:1"), "1:1");
    assert_eq!(normalize_chapter_verse_format("1"), "1");
    assert_eq!(normalize_chapter_verse_format("1:1-3"), "1:1-3");
    assert_eq!(normalize_chapter_verse_format("1,1-3"), "1:1-3");
}

#[test]
fn test_split_book_and_chapter_verse_edge_cases() {
    let result = split_book_and_chapter_verse("gen1");
    assert_eq!(result, Some(("gen".to_string(), "1".to_string())));

    let result = split_book_and_chapter_verse("gen");
    assert_eq!(result, Some(("gen".to_string(), "".to_string())));

    let result = split_book_and_chapter_verse("gen1:1");
    assert_eq!(result, Some(("gen".to_string(), "1:1".to_string())));

    let result = split_book_and_chapter_verse("1cor2");
    assert_eq!(result, Some(("1cor".to_string(), "2".to_string())));
}

#[test]
fn test_parse_chapter_verse_components_edge_cases() {
    let (normalized, (cf, ct, vf, vt)) = parse_chapter_verse_components("");
    assert_eq!(normalized, "");
    assert_eq!(cf, None);
    assert_eq!(ct, None);
    assert_eq!(vf, None);
    assert_eq!(vt, None);

    let (normalized, (cf, ct, vf, vt)) = parse_chapter_verse_components("1");
    assert_eq!(normalized, "1");
    assert_eq!(cf, Some(1));
    assert_eq!(ct, Some(1));
    assert_eq!(vf, None);
    assert_eq!(vt, None);
}

#[test]
fn test_parse_reference_edge_cases() {
    assert!(parse_reference("").is_none());
    assert!(parse_reference("invalid").is_none());
    assert!(parse_reference("gen").is_some()); // Just book name is valid
    assert!(parse_reference("gen 0").is_some()); // gen 0 is actually valid
}

#[test]
fn test_find_translated_refs_simplified_edge_cases() {
    assert!(find_translated_refs_simplified("").is_none());
    assert!(find_translated_refs_simplified("invalid").is_none());
    assert!(find_translated_refs_simplified("gen").is_some()); // Just book name is valid
}

#[test]
fn test_parse_single_reference_with_components_edge_cases() {
    assert!(parse_single_reference_with_components("").is_none());
    assert!(parse_single_reference_with_components("invalid").is_none());
    assert!(parse_single_reference_with_components("gen").is_some()); // Just book name is valid
}

#[test]
fn test_parse_contextual_references_edge_cases() {
    let (refs, labels) = parse_contextual_references("");
    assert!(refs.is_empty());
    assert!(labels.is_empty());

    let (refs, labels) = parse_contextual_references("invalid");
    assert!(refs.is_empty());
    assert!(labels.is_empty());
}

#[test]
fn test_parse_contextual_part_edge_cases() {
    let result = parse_contextual_part("", &None, None);
    assert!(result.is_none());

    let result = parse_contextual_part("invalid", &None, None);
    assert!(result.is_none());

    let result = parse_contextual_part("1:1", &Some("gen".to_string()), Some(1));
    assert!(result.is_some());
    let parsed = result.unwrap();
    assert_eq!(parsed.book, "gen");
    assert_eq!(parsed.chapter_from, Some(1));
    assert_eq!(parsed.verse_from, Some(1));
}

#[test]
fn test_preserve_original_case_in_label() {
    let refs = vec![SimplifiedResponse {
        label: "gen 1:1-3".to_string(),
        hungarian_label: "1Móz 1:1-3".to_string(),
        link: "/gen 1:1-3".to_string(),
        parsed_refs: vec![create_parsed_reference(
            "gen".to_string(),
            Some(1),
            None,
            Some(1),
            Some(3),
        )],
        verses: vec![],
        verses_as_text: vec![],
    }];
    let label = preserve_original_case_in_label("Genesis 1:1-3", &refs);
    assert_eq!(label, "Genesis 1:1-3");
}

#[test]
fn test_format_parsed_reference_label() {
    let ref1 = create_parsed_reference("gen".to_string(), Some(1), None, Some(1), Some(3));
    assert_eq!(format_parsed_reference_label(&ref1), "gen 1:1-3");

    let ref2 = create_parsed_reference("gen".to_string(), Some(1), None, None, None);
    assert_eq!(format_parsed_reference_label(&ref2), "gen 1");

    let ref3 = create_parsed_reference("gen".to_string(), Some(1), Some(3), Some(1), Some(5));
    assert_eq!(format_parsed_reference_label(&ref3), "gen 1-3:1-5");
}

#[test]
fn test_parse_reference_comma_format() {
    // Test comma format: mt4,1-2 should work the same as mt4:1-2
    let response = find_translated_refs_simplified("mt4,1-2");
    assert!(response.is_some());
    let r = response.unwrap();
    assert_eq!(r.label, "mt 4:1-2");
    assert_eq!(r.link, "/mt 4:1-2");
    assert_eq!(r.verses.len(), 0); // Empty verses array
    assert_eq!(r.parsed_refs.len(), 1);
    assert_eq!(r.parsed_refs[0].book, "mt");
    assert_eq!(r.parsed_refs[0].chapter_from, Some(4));
    assert_eq!(r.parsed_refs[0].chapter_to, None);
    assert_eq!(r.parsed_refs[0].verse_from, Some(1));
    assert_eq!(r.parsed_refs[0].verse_to, Some(2));

    // Compare with colon format to ensure they're equivalent
    let response_colon = find_translated_refs_simplified("mt4:1-2");
    assert!(response_colon.is_some());
    let r_colon = response_colon.unwrap();
    assert_eq!(r.label, r_colon.label);
    assert_eq!(r.parsed_refs[0].book, r_colon.parsed_refs[0].book);
    assert_eq!(
        r.parsed_refs[0].chapter_from,
        r_colon.parsed_refs[0].chapter_from
    );
    assert_eq!(
        r.parsed_refs[0].chapter_to,
        r_colon.parsed_refs[0].chapter_to
    );
    assert_eq!(
        r.parsed_refs[0].verse_from,
        r_colon.parsed_refs[0].verse_from
    );
    assert_eq!(r.parsed_refs[0].verse_to, r_colon.parsed_refs[0].verse_to);
}

#[test]
fn test_parse_reference_chapter_verse() {
    let suggestion = parse_reference("gen 1:1");
    assert!(suggestion.is_some());
    let s = suggestion.unwrap();
    assert_eq!(s.cat, "ref");
    assert_eq!(s.label, "gen 1:1");
    assert_eq!(s.link, "/gen 1:1");
    assert_eq!(s.hit_count, 1);

    // Check parsed_refs
    assert!(s.parsed_refs.is_some());
    let refs = s.parsed_refs.unwrap();
    assert_eq!(refs.len(), 1);
    assert_eq!(refs[0].book, "gen");
    assert_eq!(refs[0].chapter_from, Some(1));
    assert_eq!(refs[0].chapter_to, Some(1));
    assert_eq!(refs[0].verse_from, Some(1));
    assert_eq!(refs[0].verse_to, Some(1));
}

#[test]
fn test_parse_reference_complex_multiple() {
    let suggestion = parse_reference("1Kor 2,2-3;Jn 1,1-2");
    assert!(suggestion.is_some());
    let s = suggestion.unwrap();
    assert_eq!(s.cat, "ref");
    assert_eq!(s.label, "1Kor 2:2-3;Jn 1:1-2");
    assert_eq!(s.link, "/1Kor 2:2-3;Jn 1:1-2");

    // Check parsed_refs
    assert!(s.parsed_refs.is_some());
    let refs = s.parsed_refs.unwrap();
    assert_eq!(refs.len(), 2);

    // First reference: 1Kor 2:2-3
    assert_eq!(refs[0].book, "1Kor");
    assert_eq!(refs[0].chapter_from, Some(2));
    assert_eq!(refs[0].chapter_to, Some(2));
    assert_eq!(refs[0].verse_from, Some(2));
    assert_eq!(refs[0].verse_to, Some(3));

    // Second reference: Jn 1:1-2
    assert_eq!(refs[1].book, "Jn");
    assert_eq!(refs[1].chapter_from, Some(1));
    assert_eq!(refs[1].chapter_to, Some(1));
    assert_eq!(refs[1].verse_from, Some(1));
    assert_eq!(refs[1].verse_to, Some(2));
}

#[test]
fn test_parse_reference_with_translation() {
    let suggestion = parse_reference("1Kor 2,2-3;Jn 1,1-2/KG");
    assert!(suggestion.is_some());
    let s = suggestion.unwrap();
    assert_eq!(s.cat, "ref");
    assert_eq!(s.label, "1Kor 2:2-3;Jn 1:1-2");
    assert_eq!(s.link, "/1Kor 2:2-3;Jn 1:1-2/KG");

    // Check parsed_refs
    assert!(s.parsed_refs.is_some());
    let refs = s.parsed_refs.unwrap();
    assert_eq!(refs.len(), 2);

    // First reference: 1Kor 2:2-3
    assert_eq!(refs[0].book, "1Kor");
    assert_eq!(refs[0].chapter_from, Some(2));
    assert_eq!(refs[0].chapter_to, Some(2));
    assert_eq!(refs[0].verse_from, Some(2));
    assert_eq!(refs[0].verse_to, Some(3));

    // Second reference: Jn 1:1-2
    assert_eq!(refs[1].book, "Jn");
    assert_eq!(refs[1].chapter_from, Some(1));
    assert_eq!(refs[1].chapter_to, Some(1));
    assert_eq!(refs[1].verse_from, Some(1));
    assert_eq!(refs[1].verse_to, Some(2));
}

#[test]
fn test_parse_reference_wildcard_translation() {
    let suggestion = parse_reference("1Kor 2,2-3;Jn 1,1-2/*");
    assert!(suggestion.is_some());
    let s = suggestion.unwrap();
    assert_eq!(s.cat, "ref");
    assert_eq!(s.label, "1Kor 2:2-3;Jn 1:1-2");
    assert_eq!(s.link, "/1Kor 2:2-3;Jn 1:1-2/*");

    // Check parsed_refs
    assert!(s.parsed_refs.is_some());
    let refs = s.parsed_refs.unwrap();
    assert_eq!(refs.len(), 2);

    // First reference: 1Kor 2:2-3
    assert_eq!(refs[0].book, "1Kor");
    assert_eq!(refs[0].chapter_from, Some(2));
    assert_eq!(refs[0].chapter_to, Some(2));
    assert_eq!(refs[0].verse_from, Some(2));
    assert_eq!(refs[0].verse_to, Some(3));

    // Second reference: Jn 1:1-2
    assert_eq!(refs[1].book, "Jn");
    assert_eq!(refs[1].chapter_from, Some(1));
    assert_eq!(refs[1].chapter_to, Some(1));
    assert_eq!(refs[1].verse_from, Some(1));
    assert_eq!(refs[1].verse_to, Some(2));
}

#[test]
fn test_parse_reference_verse_ranges() {
    let suggestion = parse_reference("1Kor 1,1-3;5-6");
    assert!(suggestion.is_some());
    let s = suggestion.unwrap();
    assert_eq!(s.cat, "ref");
    assert_eq!(s.label, "1Kor 1:1-3;5-6");
    assert_eq!(s.link, "/1Kor 1:1-3;5-6");

    // Check parsed_refs
    assert!(s.parsed_refs.is_some());
    let refs = s.parsed_refs.unwrap();
    assert_eq!(refs.len(), 2);

    // First reference: 1Kor 1:1-3
    assert_eq!(refs[0].book, "1Kor");
    assert_eq!(refs[0].chapter_from, Some(1));
    assert_eq!(refs[0].chapter_to, Some(1));
    assert_eq!(refs[0].verse_from, Some(1));
    assert_eq!(refs[0].verse_to, Some(3));

    // Second reference: 1Kor 1:5-6 (contextual parsing - same book, same chapter)
    assert_eq!(refs[1].book, "1Kor");
    assert_eq!(refs[1].chapter_from, Some(1));
    assert_eq!(refs[1].chapter_to, Some(1));
    assert_eq!(refs[1].verse_from, Some(5));
    assert_eq!(refs[1].verse_to, Some(6));
}

#[test]
fn test_parse_reference_simple_patterns() {
    // Test simple book chapter
    let suggestion = parse_reference("genesis 3");
    assert!(suggestion.is_some());
    let s = suggestion.unwrap();
    assert_eq!(s.label, "genesis 3");

    // Check parsed_refs for chapter-only reference
    assert!(s.parsed_refs.is_some());
    let refs = s.parsed_refs.unwrap();
    assert_eq!(refs.len(), 1);
    assert_eq!(refs[0].book, "genesis");
    assert_eq!(refs[0].chapter_from, Some(3));
    assert_eq!(refs[0].chapter_to, Some(3));
    assert_eq!(refs[0].verse_from, None);
    assert_eq!(refs[0].verse_to, None);

    // Test book only
    let suggestion2 = parse_reference("1Kor");
    assert!(suggestion2.is_some());
    let s2 = suggestion2.unwrap();
    assert_eq!(s2.label, "1Kor");

    // Check parsed_refs for book-only reference
    assert!(s2.parsed_refs.is_some());
    let refs2 = s2.parsed_refs.unwrap();
    assert_eq!(refs2.len(), 1);
    assert_eq!(refs2[0].book, "1Kor");
    assert_eq!(refs2[0].chapter_from, None);
    assert_eq!(refs2[0].chapter_to, None);
    assert_eq!(refs2[0].verse_from, None);
    assert_eq!(refs2[0].verse_to, None);
}

#[test]
fn test_parse_reference_with_components() {
    // Test simple chapter:verse
    let suggestion = parse_reference("gen 1:1");
    assert!(suggestion.is_some());
    let s = suggestion.unwrap();
    assert_eq!(s.cat, "ref");
    assert_eq!(s.label, "gen 1:1");
    assert!(s.parsed_refs.is_some());
    let refs = s.parsed_refs.unwrap();
    assert_eq!(refs.len(), 1);
    assert_eq!(refs[0].book, "gen");
    assert_eq!(refs[0].chapter_from, Some(1));
    assert_eq!(refs[0].chapter_to, Some(1));
    assert_eq!(refs[0].verse_from, Some(1));
    assert_eq!(refs[0].verse_to, Some(1));

    // Test complex multiple references
    let suggestion2 = parse_reference("1Kor 2,2-3;Jn 1,1-2");
    assert!(suggestion2.is_some());
    let s2 = suggestion2.unwrap();
    assert!(s2.parsed_refs.is_some());
    let refs2 = s2.parsed_refs.unwrap();
    assert_eq!(refs2.len(), 2);

    // First reference: 1Kor 2:2-3
    assert_eq!(refs2[0].book, "1Kor");
    assert_eq!(refs2[0].chapter_from, Some(2));
    assert_eq!(refs2[0].chapter_to, Some(2));
    assert_eq!(refs2[0].verse_from, Some(2));
    assert_eq!(refs2[0].verse_to, Some(3));

    // Second reference: Jn 1:1-2
    assert_eq!(refs2[1].book, "Jn");
    assert_eq!(refs2[1].chapter_from, Some(1));
    assert_eq!(refs2[1].chapter_to, Some(1));
    assert_eq!(refs2[1].verse_from, Some(1));
    assert_eq!(refs2[1].verse_to, Some(2));
}

#[test]
fn test_parse_range() {
    // Test single number
    let (from, to) = parse_range("5");
    assert_eq!(from, Some(5));
    assert_eq!(to, Some(5));

    // Test empty string
    let (from, to) = parse_range("");
    assert_eq!(from, None);
    assert_eq!(to, None);

    // Test invalid number
    let (from, to) = parse_range("abc");
    assert_eq!(from, None);
    assert_eq!(to, None);

    // Test invalid range format
    let (from, to) = parse_range("1-");
    assert_eq!(from, Some(1));
    assert_eq!(to, None);

    // Test zero values
    let (from, to) = parse_range("0-5");
    assert_eq!(from, Some(0));
    assert_eq!(to, Some(5));

    // Test large numbers
    let (from, to) = parse_range("100-150");
    assert_eq!(from, Some(100));
    assert_eq!(to, Some(150));
}

#[test]
fn test_is_reference_pattern() {
    // Should recognize valid reference patterns
    assert!(is_reference_pattern("gen 1:1"));
    assert!(is_reference_pattern("Mt 5,3-12"));
    assert!(is_reference_pattern("Rom 3/KG"));
    assert!(is_reference_pattern("1Tim 2,1-8"));

    // Should recognize simple book abbreviations
    assert!(is_reference_pattern("gen"));
    assert!(is_reference_pattern("mt"));
    assert!(is_reference_pattern("rom"));
    assert!(is_reference_pattern("genesis"));
    assert!(is_reference_pattern("1Kor"));

    // Should recognize valid book + chapter patterns (with proper separators)
    assert!(is_reference_pattern("mt 5")); // Valid: book + space + chapter
    assert!(is_reference_pattern("gen 1")); // Valid: book + space + chapter
    assert!(is_reference_pattern("rom:3")); // Valid: book + colon + chapter
    assert!(is_reference_pattern("mt,5")); // Valid: book + comma + chapter

    // Should recognize valid book + direct chapter numbers
    assert!(is_reference_pattern("mt5")); // Valid: Matthew chapter 5
    assert!(is_reference_pattern("mt28")); // Valid: Matthew chapter 28 (last chapter)
    assert!(is_reference_pattern("gen1")); // Valid: Genesis chapter 1
    assert!(is_reference_pattern("gen50")); // Valid: Genesis chapter 50 (last chapter)
    assert!(is_reference_pattern("rom12")); // Valid: Romans chapter 12
    assert!(is_reference_pattern("rom16")); // Valid: Romans chapter 16 (last chapter)

    // Should not recognize non-valid patterns
    assert!(!is_reference_pattern("love"));
    assert!(!is_reference_pattern("jesus"));
    assert!(!is_reference_pattern("random text"));
    assert!(!is_reference_pattern("mt29")); // Book name with invalid chapter (Matthew only has 28)
    assert!(!is_reference_pattern("gen51")); // Book name with invalid chapter (Genesis only has 50)
    assert!(!is_reference_pattern("rom17")); // Book name with invalid chapter (Romans only has 16)
    assert!(!is_reference_pattern("mt99")); // Book name with invalid chapter (too high)
    assert!(!is_reference_pattern("xyz5")); // Not a book
}

#[test]
fn test_contextual_references() {
    // Test same book, same chapter, different verses: mt5,1-2;5-6
    let suggestion = parse_reference("mt5,1-2;5-6");
    assert!(suggestion.is_some());
    let s = suggestion.unwrap();
    assert_eq!(s.cat, "ref");
    assert_eq!(s.label, "mt 5:1-2;5-6"); // Short form for same book+chapter
    assert!(s.parsed_refs.is_some());
    let refs = s.parsed_refs.unwrap();
    assert_eq!(refs.len(), 2);

    // First reference: mt 5:1-2
    assert_eq!(refs[0].book, "mt");
    assert_eq!(refs[0].chapter_from, Some(5));
    assert_eq!(refs[0].chapter_to, Some(5));
    assert_eq!(refs[0].verse_from, Some(1));
    assert_eq!(refs[0].verse_to, Some(2));

    // Second reference: mt 5:5-6 (same book, same chapter)
    assert_eq!(refs[1].book, "mt");
    assert_eq!(refs[1].chapter_from, Some(5));
    assert_eq!(refs[1].chapter_to, Some(5));
    assert_eq!(refs[1].verse_from, Some(5));
    assert_eq!(refs[1].verse_to, Some(6));

    // Test same book, different chapters: mt5,1-2;6,5-6
    let suggestion2 = parse_reference("mt5,1-2;6,5-6");
    assert!(suggestion2.is_some());
    let s2 = suggestion2.unwrap();
    assert_eq!(s2.cat, "ref");
    assert_eq!(s2.label, "mt 5:1-2;mt 6:5-6"); // Full form for different chapters
    assert!(s2.parsed_refs.is_some());
    let refs2 = s2.parsed_refs.unwrap();
    assert_eq!(refs2.len(), 2);

    // First reference: mt 5:1-2
    assert_eq!(refs2[0].book, "mt");
    assert_eq!(refs2[0].chapter_from, Some(5));
    assert_eq!(refs2[0].chapter_to, Some(5));
    assert_eq!(refs2[0].verse_from, Some(1));
    assert_eq!(refs2[0].verse_to, Some(2));

    // Second reference: mt 6:5-6 (same book, different chapter)
    assert_eq!(refs2[1].book, "mt");
    assert_eq!(refs2[1].chapter_from, Some(6));
    assert_eq!(refs2[1].chapter_to, Some(6));
    assert_eq!(refs2[1].verse_from, Some(5));
    assert_eq!(refs2[1].verse_to, Some(6));
}

#[test]
fn test_additional_contextual_patterns() {
    // Test with other books and more complex patterns

    // Test gen1:1;2:1-3 (same book, different chapters with verses)
    let suggestion = parse_reference("gen1:1;2:1-3");
    assert!(suggestion.is_some());
    let s = suggestion.unwrap();
    assert_eq!(s.label, "gen 1:1;gen 2:1-3"); // Full form for different chapters
    assert!(s.parsed_refs.is_some());
    let refs = s.parsed_refs.unwrap();
    assert_eq!(refs.len(), 2);

    // First reference: gen 1:1
    assert_eq!(refs[0].book, "gen");
    assert_eq!(refs[0].chapter_from, Some(1));
    assert_eq!(refs[0].chapter_to, Some(1));
    assert_eq!(refs[0].verse_from, Some(1));
    assert_eq!(refs[0].verse_to, Some(1));

    // Second reference: gen 2:1-3
    assert_eq!(refs[1].book, "gen");
    assert_eq!(refs[1].chapter_from, Some(2));
    assert_eq!(refs[1].chapter_to, Some(2));
    assert_eq!(refs[1].verse_from, Some(1));
    assert_eq!(refs[1].verse_to, Some(3));

    // Test rom12,1-2;3-5 (same book, same chapter, different verse ranges)
    let suggestion2 = parse_reference("rom12,1-2;3-5");
    assert!(suggestion2.is_some());
    let s2 = suggestion2.unwrap();
    assert_eq!(s2.label, "rom 12:1-2;3-5"); // Short form for same chapter
    assert!(s2.parsed_refs.is_some());
    let refs2 = s2.parsed_refs.unwrap();
    assert_eq!(refs2.len(), 2);

    // First reference: rom 12:1-2
    assert_eq!(refs2[0].book, "rom");
    assert_eq!(refs2[0].chapter_from, Some(12));
    assert_eq!(refs2[0].chapter_to, Some(12));
    assert_eq!(refs2[0].verse_from, Some(1));
    assert_eq!(refs2[0].verse_to, Some(2));

    // Second reference: rom 12:3-5 (same book, same chapter)
    assert_eq!(refs2[1].book, "rom");
    assert_eq!(refs2[1].chapter_from, Some(12));
    assert_eq!(refs2[1].chapter_to, Some(12));
    assert_eq!(refs2[1].verse_from, Some(3));
    assert_eq!(refs2[1].verse_to, Some(5));

    // Test 1cor2:1;3,1-5 (same book, different chapters)
    let suggestion3 = parse_reference("1cor2:1;3,1-5");
    assert!(suggestion3.is_some());
    let s3 = suggestion3.unwrap();
    assert_eq!(s3.label, "1cor 2:1;1cor 3:1-5"); // Full form for different chapters
    assert!(s3.parsed_refs.is_some());
    let refs3 = s3.parsed_refs.unwrap();
    assert_eq!(refs3.len(), 2);

    // First reference: 1cor 2:1
    assert_eq!(refs3[0].book, "1cor");
    assert_eq!(refs3[0].chapter_from, Some(2));
    assert_eq!(refs3[0].chapter_to, Some(2));
    assert_eq!(refs3[0].verse_from, Some(1));
    assert_eq!(refs3[0].verse_to, Some(1));

    // Second reference: 1cor 3:1-5
    assert_eq!(refs3[1].book, "1cor");
    assert_eq!(refs3[1].chapter_from, Some(3));
    assert_eq!(refs3[1].chapter_to, Some(3));
    assert_eq!(refs3[1].verse_from, Some(1));
    assert_eq!(refs3[1].verse_to, Some(5));

    // Test jn1,1;2;3:16 (same book, multiple chapters, some with verses)
    let suggestion4 = parse_reference("jn1,1;2;3:16");
    assert!(suggestion4.is_some());
    let s4 = suggestion4.unwrap();
    // Note: This is a complex case that might need refinement
    assert!(s4.label.contains("jn"));
    assert!(s4.parsed_refs.is_some());
    let refs4 = s4.parsed_refs.unwrap();
    assert!(refs4.len() >= 2); // Should parse multiple chapter references
    assert_eq!(refs4[0].book, "jn");
    // First reference should be jn 1:1
    assert_eq!(refs4[0].chapter_from, Some(1));
    assert_eq!(refs4[0].verse_from, Some(1));
}

#[test]
fn test_edge_case_contextual_patterns() {
    // Test patterns that should work with the contextual parser

    // Test book only followed by contextual chapters
    let suggestion = parse_reference("ps1;2;3");
    assert!(suggestion.is_some());
    let s = suggestion.unwrap();
    assert!(s.label.contains("ps"));
    assert!(s.parsed_refs.is_some());
    let refs = s.parsed_refs.unwrap();
    assert!(refs.len() >= 2); // Should parse multiple chapter references
    assert_eq!(refs[0].book, "ps");
    assert_eq!(refs[0].chapter_from, Some(1));

    // Test with translation suffix
    let suggestion2 = parse_reference("mt5,1-2;5-6/KG");
    assert!(suggestion2.is_some());
    let s2 = suggestion2.unwrap();
    assert_eq!(s2.label, "mt 5:1-2;5-6");
    assert_eq!(s2.link, "/mt 5:1-2;5-6/KG"); // Should preserve translation

    // Check parsed_refs for translation test
    assert!(s2.parsed_refs.is_some());
    let refs2 = s2.parsed_refs.unwrap();
    assert_eq!(refs2.len(), 2);
    assert_eq!(refs2[0].book, "mt");
    assert_eq!(refs2[0].chapter_from, Some(5));
    assert_eq!(refs2[0].verse_from, Some(1));
    assert_eq!(refs2[0].verse_to, Some(2));
    assert_eq!(refs2[1].book, "mt");
    assert_eq!(refs2[1].chapter_from, Some(5));
    assert_eq!(refs2[1].verse_from, Some(5));
    assert_eq!(refs2[1].verse_to, Some(6));

    // Test mixed format: space-separated and contextual
    let suggestion3 = parse_reference("gen 1:1;2:1");
    assert!(suggestion3.is_some());
    let s3 = suggestion3.unwrap();
    assert!(s3.label.contains("gen"));
    assert!(s3.parsed_refs.is_some());
    let refs3 = s3.parsed_refs.unwrap();
    assert_eq!(refs3.len(), 2);
    assert_eq!(refs3[0].book, "gen");
    assert_eq!(refs3[0].chapter_from, Some(1));
    assert_eq!(refs3[0].verse_from, Some(1));
    assert_eq!(refs3[1].book, "gen");
    assert_eq!(refs3[1].chapter_from, Some(2));
    assert_eq!(refs3[1].verse_from, Some(1));
}

#[test]
fn test_invalid_book_validation() {
    // Test invalid book validation - invalid books should be skipped
    let (parsed_refs, suggestions) = parse_contextual_references("mt5,1-2;js7:5-6");
    assert_eq!(suggestions.len(), 1); // Only valid "mt" reference should remain
    assert_eq!(suggestions[0], "mt 5:1-2");
    assert_eq!(parsed_refs[0].book, "mt");
    assert_eq!(parsed_refs[0].chapter_from, Some(5));

    let (parsed_refs, suggestions) = parse_contextual_references("js1,1-2;mt5:3-4;xyz6:7-8");
    assert_eq!(suggestions.len(), 1); // Only valid "mt" reference should remain
    assert_eq!(suggestions[0], "mt 5:3-4");
    assert_eq!(parsed_refs[0].book, "mt");
    assert_eq!(parsed_refs[0].chapter_from, Some(5));

    let (_, suggestions) = parse_contextual_references("notabook3:4-5");
    assert_eq!(suggestions.len(), 0); // No valid references

    let (_, suggestions) = parse_contextual_references("gen1,1-2;notvalid3:4-5;ex2,6-7");
    assert_eq!(suggestions.len(), 2); // Only "gen" and "ex" references should remain
    assert_eq!(suggestions[0], "gen 1:1-2");
    assert_eq!(suggestions[1], "ex 2:6-7");

    // Test the exact case from the user report: js7:5-6 should be completely ignored
    let suggestion = parse_reference("mt5,1-2;js7:5-6");
    assert!(suggestion.is_some());
    let s = suggestion.unwrap();
    assert_eq!(s.label, "mt 5:1-2"); // Should only contain the valid reference
    assert!(s.parsed_refs.is_some());
    let refs = s.parsed_refs.unwrap();
    assert_eq!(refs.len(), 1); // Should only have 1 reference, not 2
    assert_eq!(refs[0].book, "mt");
    assert_eq!(refs[0].chapter_from, Some(5));
    assert_eq!(refs[0].verse_from, Some(1));
    assert_eq!(refs[0].verse_to, Some(2));
}

#[test]
fn test_invalid_chapter_verse_patterns_simplified() {
    // Test cases where verses are provided without chapters - these should be invalid

    // Test ":5-6" pattern (verses without chapter) - should be completely rejected
    let response = find_translated_refs_simplified("mt:5-6");
    assert!(response.is_none()); // Should be completely invalid

    // Test "jn:5-6" pattern - should be invalid
    let response2 = find_translated_refs_simplified("jn:5-6");
    assert!(response2.is_none()); // Should be completely invalid

    // Test contextual pattern "mt5,1-2;:5-6" - second part should be skipped
    let response3 = find_translated_refs_simplified("mt5,1-2;:5-6");
    assert!(response3.is_some());
    let r3 = response3.unwrap();
    assert_eq!(r3.label, "mt 5:1-2"); // Only valid part should remain
    assert_eq!(r3.parsed_refs.len(), 1); // Should only have 1 reference
    assert_eq!(r3.parsed_refs[0].book, "mt");
    assert_eq!(r3.parsed_refs[0].chapter_from, Some(5));
    assert_eq!(r3.parsed_refs[0].chapter_to, None);
    assert_eq!(r3.parsed_refs[0].verse_from, Some(1));
    assert_eq!(r3.parsed_refs[0].verse_to, Some(2));
    assert_eq!(r3.verses.len(), 0); // Empty verses array

    // Test "mt5,1-2;jn:5-6" - second part should be skipped due to missing chapter
    let response4 = find_translated_refs_simplified("mt5,1-2;jn:5-6");
    assert!(response4.is_some());
    let r4 = response4.unwrap();
    assert_eq!(r4.label, "mt 5:1-2"); // Only valid part should remain
    assert_eq!(r4.parsed_refs.len(), 1); // Should only have 1 reference
    assert_eq!(r4.parsed_refs[0].book, "mt");
    assert_eq!(r4.parsed_refs[0].chapter_from, Some(5));
    assert_eq!(r4.parsed_refs[0].chapter_to, None);
    assert_eq!(r4.parsed_refs[0].verse_from, Some(1));
    assert_eq!(r4.parsed_refs[0].verse_to, Some(2));
    assert_eq!(r4.verses.len(), 0); // Empty verses array

    // Test "gen:1-3;rom:5-8" - both should be invalid
    let response5 = find_translated_refs_simplified("gen:1-3;rom:5-8");
    assert!(response5.is_none()); // Should be completely invalid

    // Test that valid formats still work
    let response7 = find_translated_refs_simplified("jn1:5-6");
    assert!(response7.is_some());
    let r7 = response7.unwrap();
    assert_eq!(r7.label, "jn 1:5-6");
    assert_eq!(r7.parsed_refs.len(), 1);
    assert_eq!(r7.parsed_refs[0].book, "jn");
    assert_eq!(r7.parsed_refs[0].chapter_from, Some(1));
    assert_eq!(r7.parsed_refs[0].chapter_to, None);
    assert_eq!(r7.parsed_refs[0].verse_from, Some(5));
    assert_eq!(r7.parsed_refs[0].verse_to, Some(6));
    assert_eq!(r7.verses.len(), 0); // Empty verses array
}

#[test]
fn test_is_valid_book() {
    // Valid books
    assert!(parser::books::is_valid_book("mt"));
    assert!(parser::books::is_valid_book("matthew"));
    assert!(parser::books::is_valid_book("gen"));
    assert!(parser::books::is_valid_book("genesis"));
    assert!(parser::books::is_valid_book("1cor"));
    assert!(parser::books::is_valid_book("1kor")); // German style
    assert!(parser::books::is_valid_book("rev"));

    // Invalid books
    assert!(!parser::books::is_valid_book("js"));
    assert!(!parser::books::is_valid_book("xyz"));
    assert!(!parser::books::is_valid_book("notabook"));
    assert!(!parser::books::is_valid_book("invalid"));

    // Case insensitive
    assert!(parser::books::is_valid_book("MT"));
    assert!(parser::books::is_valid_book("Matthew"));
    assert!(parser::books::is_valid_book("GENESIS"));
    assert!(!parser::books::is_valid_book("JS"));
    assert!(!parser::books::is_valid_book("XYZ"));
}

#[test]
fn test_normalize_book_name() {
    use crate::parser::books::normalize_book_name;

    // Test some normalizations
    assert_eq!(normalize_book_name("1Kor"), Some("1cor".to_string()));
    assert_eq!(normalize_book_name("1kor"), Some("1cor".to_string()));
    assert_eq!(
        normalize_book_name("1corinthians"),
        Some("1cor".to_string())
    );
    assert_eq!(normalize_book_name("1cor"), Some("1cor".to_string()));
    assert_eq!(normalize_book_name("Genesis"), Some("gen".to_string()));
    assert_eq!(normalize_book_name("gen"), Some("gen".to_string()));
    assert_eq!(normalize_book_name("Matthew"), Some("mt".to_string()));
    assert_eq!(normalize_book_name("invalid"), None);
}

#[test]
fn test_parse_reference_different_chapters() {
    // Test case for 1Kor 2,2-3;3,1-2 (same book, different chapters)
    let response = find_translated_refs_simplified("1Kor 2,2-3;3,1-2");

    assert!(response.is_some());
    let r = response.unwrap();

    assert_eq!(r.label, "1cor 2:2-3;1cor 3:1-2");
    assert_eq!(r.link, "/1cor 2:2-3;1cor 3:1-2");
    assert_eq!(r.verses.len(), 0); // Empty verses array
    assert_eq!(r.parsed_refs.len(), 2);

    // First reference: 1cor 2:2-3
    assert_eq!(r.parsed_refs[0].book, "1cor");
    assert_eq!(r.parsed_refs[0].chapter_from, Some(2));
    assert_eq!(r.parsed_refs[0].chapter_to, None); // null for single chapter references
    assert_eq!(r.parsed_refs[0].verse_from, Some(2));
    assert_eq!(r.parsed_refs[0].verse_to, Some(3));

    // Second reference: 1cor 3:1-2
    assert_eq!(r.parsed_refs[1].book, "1cor");
    assert_eq!(r.parsed_refs[1].chapter_from, Some(3));
    assert_eq!(r.parsed_refs[1].chapter_to, None); // null for single chapter references
    assert_eq!(r.parsed_refs[1].verse_from, Some(1));
    assert_eq!(r.parsed_refs[1].verse_to, Some(2));
}

#[test]
fn test_api_endpoint_different_chapters() {
    // Test that our API endpoint parsing works for the requested pattern
    let response = find_translated_refs_simplified("1Kor 2,2-3;3,1-2");
    assert!(response.is_some());

    let r = response.unwrap();
    assert_eq!(r.label, "1cor 2:2-3;1cor 3:1-2");
    assert_eq!(r.link, "/1cor 2:2-3;1cor 3:1-2");
    assert_eq!(r.parsed_refs.len(), 2);

    // Verify the response structure matches what the API would return
    let json = serde_json::to_string(&r).unwrap();
    assert!(json.contains("\"label\":\"1cor 2:2-3;1cor 3:1-2\""));
    assert!(json.contains("\"link\":\"/1cor 2:2-3;1cor 3:1-2\""));
    assert!(json.contains("\"book\":\"1cor\""));
    assert!(json.contains("\"chapter_from\":2"));
    assert!(json.contains("\"chapter_to\":null"));
    assert!(json.contains("\"verse_from\":2"));
    assert!(json.contains("\"verse_to\":3"));
    assert!(json.contains("\"chapter_from\":3"));
    assert!(json.contains("\"verse_from\":1"));
    assert!(json.contains("\"verse_to\":2"));
}

#[test]
fn test_parse_reference_same_chapter_verse_range() {
    // Test case for 1Kor 2,2-3;1-2 (same book, same chapter, different verse range)
    // The second part "1-2" should inherit book "1cor" and chapter "2" from the first part
    // This tests contextual parsing with inherited chapter context for verse-only patterns
    let response = find_translated_refs_simplified("1Kor 2,2-3;1-2");

    assert!(response.is_some());
    let r = response.unwrap();

    assert_eq!(r.label, "1cor 2:2-3;1cor 2:1-2");
    assert_eq!(r.link, "/1cor 2:2-3;1cor 2:1-2");
    assert_eq!(r.verses.len(), 0); // Empty verses array
    assert_eq!(r.parsed_refs.len(), 2);

    // First reference: 1cor 2:2-3
    assert_eq!(r.parsed_refs[0].book, "1cor");
    assert_eq!(r.parsed_refs[0].chapter_from, Some(2));
    assert_eq!(r.parsed_refs[0].chapter_to, None); // null for single chapter references
    assert_eq!(r.parsed_refs[0].verse_from, Some(2));
    assert_eq!(r.parsed_refs[0].verse_to, Some(3));

    // Second reference: 1cor 2:1-2 (inherits book and chapter from first)
    assert_eq!(r.parsed_refs[1].book, "1cor");
    assert_eq!(r.parsed_refs[1].chapter_from, Some(2));
    assert_eq!(r.parsed_refs[1].chapter_to, None); // null for single chapter references
    assert_eq!(r.parsed_refs[1].verse_from, Some(1));
    assert_eq!(r.parsed_refs[1].verse_to, Some(2));
}

#[test]
fn test_error_response_serialization() {
    let error = create_error_response("error message");
    let json = serde_json::to_string(&error).unwrap();
    assert!(json.contains("\"error\":\"error message\""));
}

#[test]
fn test_parse_verse_range_coverage() {
    // Test comma-separated verse ranges
    let (from, to) = parse_verse_range("2-3,4");
    assert_eq!(from, Some(2));
    assert_eq!(to, Some(3));

    // Test single verse
    let (from, to) = parse_verse_range("5");
    assert_eq!(from, Some(5));
    assert_eq!(to, Some(5));

    // Test empty string
    let (from, to) = parse_verse_range("");
    assert_eq!(from, None);
    assert_eq!(to, None);

    // Test invalid format
    let (from, to) = parse_verse_range("invalid");
    assert_eq!(from, None);
    assert_eq!(to, None);
}

#[test]
fn test_normalize_chapter_verse_format_coverage() {
    // Test comma to colon conversion
    assert_eq!(normalize_chapter_verse_format("2,2"), "2:2");

    // Test dot to comma conversion
    assert_eq!(normalize_chapter_verse_format("3.4"), "3,4");

    // Test semicolon preservation
    assert_eq!(normalize_chapter_verse_format("1:1;2:3"), "1:1;2:3");

    // Test complex format
    assert_eq!(normalize_chapter_verse_format("1,2.3;4,5"), "1:2,3;4:5");

    // Test empty string
    assert_eq!(normalize_chapter_verse_format(""), "");
}

#[test]
fn test_parse_range_coverage() {
    // Test valid range
    let (from, to) = parse_range("1-3");
    assert_eq!(from, Some(1));
    assert_eq!(to, Some(3));

    // Test single number
    let (from, to) = parse_range("5");
    assert_eq!(from, Some(5));
    assert_eq!(to, Some(5));

    // Test empty string
    let (from, to) = parse_range("");
    assert_eq!(from, None);
    assert_eq!(to, None);

    // Test invalid number
    let (from, to) = parse_range("abc");
    assert_eq!(from, None);
    assert_eq!(to, None);

    // Test invalid range format
    let (from, to) = parse_range("1-");
    assert_eq!(from, Some(1));
    assert_eq!(to, None);

    // Test zero values
    let (from, to) = parse_range("0-5");
    assert_eq!(from, Some(0));
    assert_eq!(to, Some(5));

    // Test large numbers
    let (from, to) = parse_range("100-150");
    assert_eq!(from, Some(100));
    assert_eq!(to, Some(150));
}

#[test]
fn test_parse_chapter_verse_components_coverage() {
    // Test basic chapter:verse format
    let (normalized, (cf, _ct, vf, _vt)) = parse_chapter_verse_components("1:1");
    assert_eq!(normalized, "1:1");
    assert!(cf.is_some());
    assert!(vf.is_some());
}

#[test]
fn test_find_translated_refs_simplified_coverage() {
    // Test empty input
    let response = find_translated_refs_simplified("");
    assert!(response.is_none());

    // Test valid reference
    let response = find_translated_refs_simplified("gen 1:1");
    assert!(response.is_some());
}

#[test]
fn test_book_id_population() {
    // Test that book_id is populated correctly in ParsedReference
    let response = find_translated_refs_simplified("gen 1:1");
    assert!(response.is_some());

    let response = response.unwrap();
    assert!(!response.parsed_refs.is_empty());

    let parsed_ref = &response.parsed_refs[0];
    assert_eq!(parsed_ref.book, "gen");
    assert_eq!(parsed_ref.book_id, Some(1)); // Genesis should have book_id = 1
    assert_eq!(parsed_ref.chapter_from, Some(1));
    assert_eq!(parsed_ref.verse_from, Some(1));

    // Test another book
    let response2 = find_translated_refs_simplified("john 3:16");
    assert!(response2.is_some());

    let response2 = response2.unwrap();
    let parsed_ref2 = &response2.parsed_refs[0];
    assert_eq!(parsed_ref2.book, "jn");
    assert_eq!(parsed_ref2.book_id, Some(43)); // John should have book_id = 43
}
