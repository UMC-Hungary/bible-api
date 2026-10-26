#![allow(dead_code)]

use worker::*;

mod db;
mod parser;
mod response_builders;
mod types;

#[cfg(test)]
mod lib_spec;

// Re-export types for external use
pub use types::*;

pub use response_builders::*;

// Search suggestions handler - similar to PHP anySuggest() method
// Provides autocomplete/suggestion functionality for search terms
//
// Parameters (in order):
// 1. term (required): Bible reference search term
// 2. language/translation (optional): Translation code (e.g., "UF", "RUF")
//    - UF: Újfordítású Biblia / New Translation (default, trans=0)
//    - RUF: Revised Újfordítású Biblia (trans=1)
//
// Supported routes:
// - /suggest/{term} (defaults to UF translation)
// - /suggest/{term}/{translation} (specify translation via path param)
//
// Returns simplified response with label, link, parsed_refs, and verses from database
//
// Example: /suggest/mt1,1-2/UF or /suggest/1Kor%202,2-3.4;Jn%201,1-2/RUF
pub async fn search_suggestions_handler(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let url = req.url()?;
    let params: std::collections::HashMap<String, String> =
        url.query_pairs().into_owned().collect();

    // Get term from path parameter first, then query param
    let term = if let Some(term_param) = ctx.param("term") {
        urlencoding::decode(term_param)
            .unwrap_or_else(|_| term_param.into())
            .to_string()
    } else {
        params.get("term").cloned().unwrap_or_default()
    };

    // Get translation from path parameter, then query param, then default to UF
    let translation = if let Some(trans_param) = ctx.param("translation") {
        trans_param.to_uppercase()
    } else {
        params
            .get("translation")
            .cloned()
            .unwrap_or_else(|| "UF".to_string())
            .to_uppercase()
    };

    if term.is_empty() {
        let error_response = create_error_response("Missing search term");
        return Ok(Response::from_json(&error_response)?.with_status(400));
    }

    // Try to find reference matches
    if let Some(mut response) = find_translated_refs_simplified(&term) {
        // If we found valid references, query the database for actual verses
        if !response.parsed_refs.is_empty() {
            match db::query_verses_from_database(&ctx, &response.parsed_refs, &translation).await {
                Ok((verse_objects, verse_texts)) => {
                    response.verses = verse_objects;
                    response.verses_as_text = verse_texts;
                }
                Err(_) => {
                    // If database query fails, try a simpler approach
                    for parsed_ref in &response.parsed_refs {
                        if let Some(chapter) = parsed_ref.chapter_from {
                            // Try to get the verse text directly (works for both specific verses and entire chapters)
                            if let Ok((verse_objects, combined_text)) =
                                db::get_verse_text_with_translation(&ctx, parsed_ref, &translation)
                                    .await
                            {
                                response.verses.extend(verse_objects);
                                response.verses_as_text.push(combined_text);
                            } else {
                                // Create placeholder based on what was requested
                                if let Some(verse) = parsed_ref.verse_from {
                                    let placeholder = format!(
                                        "{} {}:{} - [Translation text would go here]",
                                        parsed_ref.book, chapter, verse
                                    );
                                    response.verses_as_text.push(placeholder);
                                } else {
                                    let placeholder = format!(
                                        "{} {} - [Chapter text would go here]",
                                        parsed_ref.book, chapter
                                    );
                                    response.verses_as_text.push(placeholder);
                                }
                            }
                        }
                    }
                }
            }
        }
        return Response::from_json(&response);
    }

    // If no references found, return empty response
    let empty_response = SimplifiedResponse {
        label: String::new(),
        hungarian_label: String::new(),
        link: String::new(),
        parsed_refs: Vec::new(),
        verses: Vec::new(),
        verses_as_text: Vec::new(),
    };
    Response::from_json(&empty_response)
}

// Swagger UI handler - serves the generated Redoc documentation
pub async fn swagger_handler(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let api_docs_html = include_str!("../api-docs.html");
    Response::from_html(api_docs_html)
}

// OpenAPI spec handler - serves the raw YAML
pub async fn openapi_handler(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let openapi_spec = include_str!("../openapi.yaml");

    let mut response = Response::ok(openapi_spec)?;
    response
        .headers_mut()
        .set("Content-Type", "application/x-yaml")?;
    Ok(response)
}

// Find translated references - simplified version for new response format
fn find_translated_refs_simplified(term: &str) -> Option<SimplifiedResponse> {
    let results = parser::parser::smart_parse_reference(term);

    if results.is_empty() {
        return None;
    }

    if results.len() == 1 {
        // Single result, return as-is
        return Some(results.into_iter().next().unwrap());
    }

    // Multiple results, combine them
    let mut all_parsed_refs = Vec::new();
    let mut label_parts = Vec::new();
    let mut hungarian_label_parts = Vec::new();

    for result in results {
        all_parsed_refs.extend(result.parsed_refs);
        label_parts.push(result.label);
        hungarian_label_parts.push(result.hungarian_label);
    }

    let combined_label = label_parts.join(";");
    let combined_hungarian_label = hungarian_label_parts.join(";");
    let combined_link = format!("/{combined_label}");

    Some(SimplifiedResponse {
        label: combined_label,
        hungarian_label: combined_hungarian_label,
        link: combined_link,
        parsed_refs: all_parsed_refs,
        verses: Vec::new(),
        verses_as_text: Vec::new(),
    })
}

// Helper function to parse references like "gen 1:1", "genesis 1", etc.
// Also handles complex patterns like:
// - "1Kor 2,2-3;Jn 1,1-2" (multiple references with semicolons)
// - "1Kor 2,2-3;Jn 1,1-2/KG" (with translation suffix)
// - "1Kor 1,1-3;5-6" (verse ranges)
// - "mt5,1-2;5-6" (same book, same chapter, different verses)
// - "mt5,1-2;6,5-6" (same book, different chapters)

// Check if the input looks like a reference pattern
#[allow(dead_code)]
fn is_reference_pattern(term: &str) -> bool {
    // Look for patterns that suggest references:
    // - Numbers followed by book names (1Kor, 2Tim, etc.)
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
    let books = parser::books::get_books();

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

// Extract translation suffix like "/KG" or "/*"
#[cfg(test)]
fn extract_translation_suffix(term: &str) -> (&str, Option<&str>) {
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

// Parse references with context awareness for semicolon-separated parts
fn parse_contextual_references(main_ref: &str) -> (Vec<ParsedReference>, Vec<String>) {
    let references: Vec<&str> = main_ref.split(';').collect();
    let mut parsed_refs = Vec::new();
    let mut normalized_labels = Vec::new();
    let mut current_book: Option<String> = None;
    let mut current_chapter: Option<u32> = None;

    for ref_part in references {
        let ref_part = ref_part.trim();

        // Try to parse as a complete reference first (original behavior)
        if let Some((normalized_label, parsed_ref)) =
            parse_single_reference_with_components(ref_part)
        {
            // First, always validate that the book is valid
            if !parser::books::is_valid_book(&parsed_ref.book) {
                // Skip this reference entirely if the book is not valid
                continue;
            }

            // Check if this looks like a complete reference with a proper book name
            let books = parser::books::get_books();
            let is_complete_reference = books.iter().any(|(book_names, _)| {
                book_names
                    .iter()
                    .any(|&book| ref_part.to_lowercase().starts_with(book))
            });

            if is_complete_reference || parsed_refs.is_empty() {
                // This is a complete reference or the first reference, update our context
                current_book = Some(parsed_ref.book.clone());
                current_chapter = parsed_ref.chapter_from;
                normalized_labels.push(normalized_label);
                parsed_refs.push(parsed_ref);
            } else {
                // This might be a partial reference, try contextual parsing
                if let Some(contextual_ref) =
                    parse_contextual_part(ref_part, &current_book, current_chapter)
                {
                    // Validate that the contextual book is still valid
                    if parser::books::is_valid_book(&contextual_ref.book) {
                        // For contextual references, use shorter labels when context is the same
                        let contextual_label = if Some(&contextual_ref.book)
                            == current_book.as_ref()
                            && contextual_ref.chapter_from == current_chapter
                        {
                            // Same book and chapter, just show verse range
                            format_verse_range_only(&contextual_ref)
                        } else {
                            // Different book or chapter, show full reference
                            format_parsed_reference_label(&contextual_ref)
                        };

                        // Update current chapter if this reference specifies a new one
                        if contextual_ref.chapter_from.is_some() {
                            current_chapter = contextual_ref.chapter_from;
                        }
                        normalized_labels.push(contextual_label);
                        parsed_refs.push(contextual_ref);
                    }
                    // Skip if not a valid book
                } else {
                    // Fallback to treating it as a complete reference, but validate first
                    if parser::books::is_valid_book(&parsed_ref.book) {
                        normalized_labels.push(normalized_label);
                        parsed_refs.push(parsed_ref);
                    }
                    // Skip if not a valid book
                }
            }
        } else {
            // This might be a partial reference (just chapter/verse info)
            // Try to parse it in the context of the current book and chapter
            if let Some(contextual_ref) =
                parse_contextual_part(ref_part, &current_book, current_chapter)
            {
                // Validate that the contextual book is still valid
                if parser::books::is_valid_book(&contextual_ref.book) {
                    // For contextual references, use shorter labels when context is the same
                    let contextual_label = if Some(&contextual_ref.book) == current_book.as_ref()
                        && contextual_ref.chapter_from == current_chapter
                    {
                        // Same book and chapter, just show verse range
                        format_verse_range_only(&contextual_ref)
                    } else {
                        // Different book or chapter, show full reference
                        format_parsed_reference_label(&contextual_ref)
                    };

                    // Update current chapter if this reference specifies a new one
                    if contextual_ref.chapter_from.is_some() {
                        current_chapter = contextual_ref.chapter_from;
                    }
                    normalized_labels.push(contextual_label);
                    parsed_refs.push(contextual_ref);
                }
                // Skip if not a valid book
            }
            // Skip if unable to parse at all
        }
    }

    (parsed_refs, normalized_labels)
}

// Helper function to create ParsedReference with book_id
fn create_parsed_reference(
    book: String,
    chapter_from: Option<u32>,
    chapter_to: Option<u32>,
    verse_from: Option<u32>,
    verse_to: Option<u32>,
) -> ParsedReference {
    let book_id = db::get_book_id_from_name(&book);
    ParsedReference {
        book,
        book_id,
        chapter_from,
        chapter_to,
        verse_from,
        verse_to,
    }
}

// Parse a partial reference in the context of a current book and chapter
fn parse_contextual_part(
    part: &str,
    current_book: &Option<String>,
    current_chapter: Option<u32>,
) -> Option<ParsedReference> {
    if let Some(book) = current_book {
        // Check if this is just verse numbers (like "5-6" in "mt5,1-2;5-6")
        if part.chars().all(|c| c.is_ascii_digit() || c == '-') {
            // This is likely just verse numbers for the same chapter
            if let Some(chapter) = current_chapter {
                let (verse_from, verse_to) = parse_range(part);
                return Some(create_parsed_reference(
                    book.clone(),
                    Some(chapter),
                    Some(chapter),
                    verse_from,
                    verse_to,
                ));
            }
        }

        // Check if this is chapter:verse format (like "2:1-3" in "gen1:1;2:1-3")
        if let Some(colon_pos) = part.find(':') {
            let chapter_part = &part[..colon_pos];
            let verse_part = &part[colon_pos + 1..];

            if chapter_part.is_empty() {
                // Invalid: ":5-6" format without chapter
                return None;
            }

            if let Ok(chapter) = chapter_part.parse::<u32>() {
                let (verse_from, verse_to) = parse_range(verse_part);
                return Some(create_parsed_reference(
                    book.clone(),
                    Some(chapter),
                    Some(chapter),
                    verse_from,
                    verse_to,
                ));
            }
        }

        // Check if this is chapter,verse format (like "6,5-6" in "mt5,1-2;6,5-6")
        if let Some(comma_pos) = part.find(',') {
            let chapter_part = &part[..comma_pos];
            let verse_part = &part[comma_pos + 1..];

            if let Ok(chapter) = chapter_part.parse::<u32>() {
                let (verse_from, verse_to) = parse_range(verse_part);
                return Some(create_parsed_reference(
                    book.clone(),
                    Some(chapter),
                    Some(chapter),
                    verse_from,
                    verse_to,
                ));
            }
        }

        // Check if this is just a chapter number
        if let Ok(chapter) = part.parse::<u32>() {
            return Some(create_parsed_reference(
                book.clone(),
                Some(chapter),
                Some(chapter),
                None,
                None,
            ));
        }
    }

    None
}

// Format a parsed reference back to a normalized label
fn format_parsed_reference_label(parsed_ref: &ParsedReference) -> String {
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

// Format just the verse range part for contextual references
fn format_verse_range_only(parsed_ref: &ParsedReference) -> String {
    if let Some(verse_from) = parsed_ref.verse_from {
        if let Some(verse_to) = parsed_ref.verse_to {
            if verse_from == verse_to {
                format!("{verse_from}")
            } else {
                format!("{verse_from}-{verse_to}")
            }
        } else {
            format!("{verse_from}")
        }
    } else {
        // If no verses, might be just a chapter
        if let Some(chapter_from) = parsed_ref.chapter_from {
            if let Some(chapter_to) = parsed_ref.chapter_to {
                if chapter_from == chapter_to {
                    format!("{chapter_from}")
                } else {
                    format!("{chapter_from}-{chapter_to}")
                }
            } else {
                format!("{chapter_from}")
            }
        } else {
            String::new()
        }
    }
}

// Parse a single reference like "1Kor 2,2-3" or "Jn 1,1-2" and return both normalized string and parsed components
fn parse_single_reference_with_components(ref_str: &str) -> Option<(String, ParsedReference)> {
    let ref_str = ref_str.trim();
    if ref_str.is_empty() {
        return None;
    }

    // Check if this is a direct book+number pattern like "mt5", "gen12", etc.
    if let Some((book, chapter_verse_part)) = split_book_and_chapter_verse(ref_str) {
        // Validate the book before proceeding
        if !parser::books::is_valid_book(&book) {
            return None;
        }

        // We have a book and chapter/verse part
        if chapter_verse_part.is_empty() {
            // Just a book name
            let parsed_ref = create_parsed_reference(book.clone(), None, None, None, None);
            return Some((book, parsed_ref));
        } else {
            // Parse the chapter:verse part
            let (normalized, parsed_components) =
                parse_chapter_verse_components(&chapter_verse_part);

            let parsed_ref = create_parsed_reference(
                book.clone(),
                parsed_components.0,
                parsed_components.1,
                parsed_components.2,
                parsed_components.3,
            );

            // Validate that if verses are present, chapter must also be present
            if (parsed_ref.verse_from.is_some() || parsed_ref.verse_to.is_some())
                && parsed_ref.chapter_from.is_none()
            {
                return None; // Invalid: verses without chapter
            }

            let normalized_label = format!("{book} {normalized}");
            return Some((normalized_label, parsed_ref));
        }
    }

    // Fallback to original whitespace-based parsing
    let parts: Vec<&str> = ref_str.split_whitespace().collect();
    if parts.is_empty() {
        return None;
    }

    let book = parts[0].to_string();

    // Validate the book before proceeding
    if !parser::books::is_valid_book(&book) {
        return None;
    }

    if parts.len() == 1 {
        // Just a book name
        let parsed_ref = create_parsed_reference(book.clone(), None, None, None, None);
        return Some((book, parsed_ref));
    }

    if parts.len() >= 2 {
        let chapter_verse_part = parts[1..].join(" ");

        // Parse the chapter:verse part
        let (normalized, parsed_components) = parse_chapter_verse_components(&chapter_verse_part);

        let parsed_ref = create_parsed_reference(
            book.clone(),
            parsed_components.0,
            parsed_components.1,
            parsed_components.2,
            parsed_components.3,
        );

        // Validate that if verses are present, chapter must also be present
        if (parsed_ref.verse_from.is_some() || parsed_ref.verse_to.is_some())
            && parsed_ref.chapter_from.is_none()
        {
            return None; // Invalid: verses without chapter
        }

        let normalized_label = format!("{book} {normalized}");
        return Some((normalized_label, parsed_ref));
    }

    let parsed_ref = create_parsed_reference(book.clone(), None, None, None, None);
    Some((ref_str.to_string(), parsed_ref))
}

// Split a reference like "mt5" into ("mt", "5") or "1Kor2" into ("1Kor", "2")
fn split_book_and_chapter_verse(ref_str: &str) -> Option<(String, String)> {
    // Get books to check against
    let books = parser::books::get_books();

    let ref_lower = ref_str.to_lowercase();

    // Check each book pattern to see if the reference starts with it
    for (book_names, _) in &books {
        for &book in book_names {
            if ref_lower.starts_with(book) {
                let after_book = &ref_str[book.len()..];

                // If there's content after the book name, split it
                if !after_book.is_empty() {
                    // Check if the content after book is numeric or contains chapter/verse patterns
                    if after_book
                        .chars()
                        .all(|c| c.is_ascii_digit() || c == ':' || c == ',' || c == '-' || c == '.')
                    {
                        // Return the original book name from the input, preserving case
                        let original_book = &ref_str[0..book.len()];
                        return Some((original_book.to_string(), after_book.to_string()));
                    }
                } else {
                    // Just the book name - return the original case
                    let original_book = &ref_str[0..book.len()];
                    return Some((original_book.to_string(), String::new()));
                }
            }
        }
    }

    None
}

type ChapterVerseResult = (String, (Option<u32>, Option<u32>, Option<u32>, Option<u32>));
fn parse_chapter_verse_components(cv_str: &str) -> ChapterVerseResult {
    // Handle patterns like:
    // "2,2-3.4" -> chapter 2, verses 2-3 and 4
    // "1,1-2" -> chapter 1, verses 1-2
    // "3" -> chapter 3
    // "1-3" -> chapters 1-3

    let normalized = normalize_chapter_verse_format(cv_str);

    // Parse the normalized format
    if let Some(colon_pos) = normalized.find(':') {
        // Has verses: "2:2-3,4" or "1:1-2"
        let chapter_part = &normalized[..colon_pos];
        let verse_part = &normalized[colon_pos + 1..];

        let (chapter_from, chapter_to) = parse_range(chapter_part);
        let (verse_from, verse_to) = parse_verse_range(verse_part);

        (normalized, (chapter_from, chapter_to, verse_from, verse_to))
    } else {
        // Only chapter: "3" or "1-3"
        let (chapter_from, chapter_to) = parse_range(&normalized);
        (normalized, (chapter_from, chapter_to, None, None))
    }
}

// Parse a range like "1-3" or single number "5"
fn parse_range(range_str: &str) -> (Option<u32>, Option<u32>) {
    if let Some(dash_pos) = range_str.find('-') {
        // Range: "1-3"
        let from_str = &range_str[..dash_pos];
        let to_str = &range_str[dash_pos + 1..];

        let from = from_str.parse::<u32>().ok();
        let to = to_str.parse::<u32>().ok();
        (from, to)
    } else {
        // Single number: "5"
        let num = range_str.parse::<u32>().ok();
        (num, num)
    }
}

// Parse verse ranges that can include multiple comma-separated parts like "2-3,4"
fn parse_verse_range(verse_str: &str) -> (Option<u32>, Option<u32>) {
    // For now, just handle the first part before comma
    // "2-3,4" -> just parse "2-3"
    let first_part = verse_str.split(',').next().unwrap_or(verse_str);
    parse_range(first_part)
}

// Normalize chapter/verse format to a consistent pattern
fn normalize_chapter_verse_format(cv_str: &str) -> String {
    // Convert commas to colons for standard format
    // Handle ranges and multiple verse groups
    // Convert commas to colons for standard format
    // Handle ranges and multiple verse groups
    cv_str
        .replace(',', ":") // 2,2 -> 2:2
        .replace('.', ",") // 3.4 -> 3,4 (for multiple verses in same chapter)
                           // Keep semicolons for multiple ranges
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    console_error_panic_hook::set_once();
    let allowed_origins = env.var("ALLOWED_ORIGINS")?.to_string();

    let origin = req.headers().get("Origin")?.unwrap_or_default();
    let allowed_origin = if allowed_origins.split(',').any(|o| o.trim() == origin) {
        origin
    } else {
        "*".to_string()
    };

    // Handle CORS preflight requests
    if req.method() == Method::Options {
        let mut response = Response::empty()?;
        let headers = response.headers_mut();
        headers.set("Access-Control-Allow-Origin", &allowed_origin)?;
        headers.set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")?;
        headers.set("Access-Control-Allow-Headers", "Content-Type")?;
        headers.set("Access-Control-Max-Age", "86400")?;
        return Ok(response);
    }

    let mut response = Router::new()
        .get_async("/suggest/:term", search_suggestions_handler)
        .get_async("/suggest/:term/:translation", search_suggestions_handler)
        .get_async("/swagger", swagger_handler)
        .get_async("/openapi.yaml", openapi_handler)
        .run(req, env)
        .await?;

    // Add CORS headers
    let headers = response.headers_mut();
    headers.set("Access-Control-Allow-Origin", &allowed_origin)?;
    headers.set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")?;
    headers.set("Access-Control-Allow-Headers", "Content-Type")?;
    if allowed_origin != "*" {
        headers.set("Access-Control-Allow-Credentials", "true")?;
    }

    Ok(response)
}

// Stub handlers for queue and scheduled events to provide exports
// This worker doesn't use these features but exports are required by the shim
#[event(queue)]
pub async fn queue(
    _batch: MessageBatch<serde_json::Value>,
    _env: Env,
    _ctx: worker::Context,
) -> Result<()> {
    Ok(())
}

#[event(scheduled)]
pub async fn scheduled(_event: ScheduledEvent, _env: Env, _ctx: ScheduleContext) {
    // No-op: this worker doesn't handle scheduled events
}
