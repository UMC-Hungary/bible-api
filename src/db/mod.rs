// Database access functions for Bible verses

use crate::types::{ParsedReference, VerseResponse};
use worker::*;

#[cfg(test)]
mod db_spec;

/// Map translation code to database trans ID
/// - UF: trans=0 (Újfordítású Biblia)
/// - RUF: trans=1 (Revideált Új Fordítás)
fn get_trans_id(translation: &str) -> i32 {
    match translation.to_uppercase().as_str() {
        "RUF" => 1,
        "UF" => 0,
        _ => 0, // Default to UF (trans=0)
    }
}

/// Query Bible verses from database based on parsed references
pub async fn query_verses_from_database(
    ctx: &RouteContext<()>,
    parsed_refs: &[ParsedReference],
    translation: &str,
) -> Result<(Vec<VerseResponse>, Vec<String>)> {
    let db = ctx.env.d1("SEARCH_DB")?;
    let verse_objects = Vec::new();
    let verse_texts = Vec::new();

    for parsed_ref in parsed_refs {
        // Use the book_id that was already selected by the parser
        let book_id = match parsed_ref.book_id {
            Some(id) => id,
            None => continue, // Skip if book_id not found
        };

        // Build the verse query based on the reference
        let trans_id = get_trans_id(translation);
        let (verse_query, params) = build_verse_query(book_id, parsed_ref, trans_id);
        let mut stmt = db.prepare(&verse_query);

        // Bind parameters
        for param in params {
            stmt = stmt.bind_refs(&[param])?;
        }

        // For now, skip the complex row access and let the error handling take care of it
        // This will trigger the fallback to get_verse_text_simple
        return Err(worker::Error::RustError(
            "Using simple approach".to_string(),
        ));
    }

    Ok((verse_objects, verse_texts))
}

/// Build optimized SQL query for verses based on parsed reference
fn build_optimized_verse_query(
    book_id: i32,
    parsed_ref: &ParsedReference,
    trans_id: i32,
) -> (String, Vec<worker::d1::D1Type<'_>>) {
    let mut query = String::from(
        "SELECT bvt.text, bv.chapter, bv.verse FROM bible_verses bv 
         LEFT JOIN bible_verse_translation bvt ON bv.id = bvt.bible_verses_id 
         WHERE bv.bible_books_id = ?1 AND bvt.trans = ?2",
    );
    let mut params = vec![
        worker::d1::D1Type::Integer(book_id),
        worker::d1::D1Type::Integer(trans_id),
    ];
    let mut param_count = 2;

    // Handle chapter conditions
    if let Some(chapter_from) = parsed_ref.chapter_from {
        if let Some(chapter_to) = parsed_ref.chapter_to {
            if chapter_from == chapter_to {
                // Single chapter
                param_count += 1;
                query.push_str(&format!(" AND bv.chapter = ?{}", param_count));
                params.push(worker::d1::D1Type::Integer(chapter_from as i32));
            } else {
                // Chapter range
                param_count += 1;
                query.push_str(&format!(" AND bv.chapter >= ?{}", param_count));
                params.push(worker::d1::D1Type::Integer(chapter_from as i32));
                param_count += 1;
                query.push_str(&format!(" AND bv.chapter <= ?{}", param_count));
                params.push(worker::d1::D1Type::Integer(chapter_to as i32));
            }
        } else {
            // Single chapter (no chapter_to)
            param_count += 1;
            query.push_str(&format!(" AND bv.chapter = ?{}", param_count));
            params.push(worker::d1::D1Type::Integer(chapter_from as i32));
        }
    }

    // Handle verse conditions
    if let Some(verse_from) = parsed_ref.verse_from {
        if let Some(verse_to) = parsed_ref.verse_to {
            if verse_from == verse_to {
                // Single verse
                param_count += 1;
                query.push_str(&format!(" AND bv.verse = ?{}", param_count));
                params.push(worker::d1::D1Type::Integer(verse_from as i32));
            } else {
                // Verse range
                param_count += 1;
                query.push_str(&format!(" AND bv.verse >= ?{}", param_count));
                params.push(worker::d1::D1Type::Integer(verse_from as i32));
                param_count += 1;
                query.push_str(&format!(" AND bv.verse <= ?{}", param_count));
                params.push(worker::d1::D1Type::Integer(verse_to as i32));
            }
        } else {
            // Single verse (no verse_to)
            param_count += 1;
            query.push_str(&format!(" AND bv.verse = ?{}", param_count));
            params.push(worker::d1::D1Type::Integer(verse_from as i32));
        }
    }

    // Add ordering
    query.push_str(" ORDER BY bv.chapter, bv.verse");

    (query, params)
}

/// Build SQL query for verses based on parsed reference (legacy function)
fn build_verse_query(
    book_id: i32,
    parsed_ref: &ParsedReference,
    trans_id: i32,
) -> (String, Vec<worker::d1::D1Type<'_>>) {
    let mut query = String::from(
        "SELECT bvt.text FROM bible_verses bv 
         LEFT JOIN bible_verse_translation bvt ON bv.id = bvt.bible_verses_id 
         WHERE bv.bible_books_id = ?1 AND bvt.trans = ?2",
    );
    let mut params = vec![
        worker::d1::D1Type::Integer(book_id),
        worker::d1::D1Type::Integer(trans_id),
    ];
    let mut param_count = 2;

    // Add chapter filter
    if let Some(chapter) = parsed_ref.chapter_from {
        param_count += 1;
        query.push_str(&format!(" AND bv.chapter = ?{}", param_count));
        params.push(worker::d1::D1Type::Integer(chapter as i32));

        // Add verse filter if specified
        if let Some(verse_from) = parsed_ref.verse_from {
            param_count += 1;
            if let Some(verse_to) = parsed_ref.verse_to {
                if verse_from == verse_to {
                    query.push_str(&format!(" AND bv.verse = ?{}", param_count));
                    params.push(worker::d1::D1Type::Integer(verse_from as i32));
                } else {
                    query.push_str(&format!(
                        " AND bv.verse >= ?{} AND bv.verse <= ?{}",
                        param_count,
                        param_count + 1
                    ));
                    params.push(worker::d1::D1Type::Integer(verse_from as i32));
                    params.push(worker::d1::D1Type::Integer(verse_to as i32));
                }
            } else {
                query.push_str(&format!(" AND bv.verse = ?{}", param_count));
                params.push(worker::d1::D1Type::Integer(verse_from as i32));
            }
        }
    }

    query.push_str(" ORDER BY bv.chapter, bv.verse");

    (query, params)
}

use crate::parser::book_config::get_book_configs;

/// Book name to database ID mapping
/// Supports English and Hungarian book names with common synonyms
pub fn get_book_id_from_name(book_name: &str) -> Option<i32> {
    let book_lower = book_name.to_lowercase();

    // Use centralized book configuration
    for config in get_book_configs() {
        if config.names.iter().any(|&name| name == book_lower) {
            return Some(config.id);
        }
    }

    None
}

/// Sanitize text by removing HTML tags and their content, then cleaning up whitespace
fn sanitize_text(text: &str) -> String {
    let mut cleaned = text.to_string();

    // Remove heading tags with their content
    for tag in &["h1", "h2", "h3", "h4", "h5", "h6"] {
        let pattern = format!(r"<{}[^>]*>.*?</{}>", tag, tag);
        let re = regex::Regex::new(&pattern).unwrap();
        cleaned = re.replace_all(&cleaned, "").to_string();
    }

    // Remove other block elements with their content
    for tag in &["span", "div", "p", "section", "header", "footer"] {
        let pattern = format!(r"<{}[^>]*>.*?</{}>", tag, tag);
        let re = regex::Regex::new(&pattern).unwrap();
        cleaned = re.replace_all(&cleaned, "").to_string();
    }

    // Remove link tags but keep the text inside
    let link_pattern = regex::Regex::new(r"<a[^>]*>(.*?)</a>").unwrap();
    cleaned = link_pattern.replace_all(&cleaned, "$1").to_string();

    // Remove any remaining HTML tags (like <br>, <hr>, etc.)
    let remaining_tags = regex::Regex::new(r"<[^>]+>").unwrap();
    cleaned = remaining_tags.replace_all(&cleaned, "").to_string();

    // Clean up multiple spaces and normalize whitespace
    let whitespace_pattern = regex::Regex::new(r"\s+").unwrap();
    cleaned = whitespace_pattern.replace_all(&cleaned, " ").to_string();

    // Trim leading and trailing whitespace
    cleaned.trim().to_string()
}

/// Simple function to get verse text using direct book ID mapping
/// ONE SQL query per parsed reference - fetches all matching verses at once
pub async fn get_verse_text_simple(
    ctx: &RouteContext<()>,
    parsed_ref: &ParsedReference,
) -> Result<(Vec<VerseResponse>, String)> {
    get_verse_text_with_translation(ctx, parsed_ref, "UF").await
}

/// Get verse text with specific translation
pub async fn get_verse_text_with_translation(
    ctx: &RouteContext<()>,
    parsed_ref: &ParsedReference,
    translation: &str,
) -> Result<(Vec<VerseResponse>, String)> {
    let db = ctx.env.d1("SEARCH_DB")?;

    // Use the book_id that was already selected by the parser
    let book_id = match parsed_ref.book_id {
        Some(id) => id,
        None => {
            return Err(worker::Error::RustError(format!(
                "Book '{}' not found in mapping",
                parsed_ref.book
            )))
        }
    };

    let trans_id = get_trans_id(translation);

    // Build ONE query that fetches all matching verses using the optimized builder
    let (query, params) = build_optimized_verse_query(book_id, parsed_ref, trans_id);

    // Prepare statement with all parameters at once
    let stmt = db.prepare(&query).bind_refs(params.as_slice())?;

    // Execute the single query and collect all results
    let results = stmt.all().await?;
    let results_value = results.results::<serde_json::Value>()?;

    let mut verse_objects = Vec::new();
    let mut verse_texts = Vec::new();

    for row in results_value {
        // Extract fields from the row
        let text_value = row.get("text");
        let chapter_value = row.get("chapter");
        let verse_value = row.get("verse");

        if let (Some(text), Some(chapter), Some(verse)) = (text_value, chapter_value, verse_value) {
            if let (Some(text_str), Some(chapter_num), Some(verse_num)) =
                (text.as_str(), chapter.as_i64(), verse.as_i64())
            {
                let sanitized_text = sanitize_text(text_str);
                verse_objects.push(VerseResponse {
                    chapter: chapter_num as i32,
                    verse: verse_num as i32,
                    text: sanitized_text.clone(),
                });
                verse_texts.push(sanitized_text);
            }
        }
    }

    if verse_objects.is_empty() {
        Err(worker::Error::RustError("No verses found".to_string()))
    } else {
        let combined_text = verse_texts.join(" ");
        Ok((verse_objects, combined_text))
    }
}
