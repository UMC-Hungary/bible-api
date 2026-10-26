use super::book_config::get_book_configs;
use super::hungarian_names::get_hungarian_abbrev;

pub fn get_books() -> Vec<(Vec<&'static str>, u32)> {
    get_book_configs()
        .into_iter()
        .map(|config| (config.names, config.chapters))
        .collect()
}

pub fn is_valid_book(book: &str) -> bool {
    let book_lower = book.to_lowercase();
    let books = get_books();

    books.iter().any(|(book_names, _)| {
        book_names
            .iter()
            .any(|&valid_book| valid_book == book_lower)
    })
}

pub fn normalize_book_name(book: &str) -> Option<String> {
    let book_lower = book.to_lowercase();
    let books = get_books();

    for (book_names, _) in &books {
        if book_names
            .iter()
            .any(|&valid_book| valid_book == book_lower)
        {
            // Return the first (canonical short) form
            return Some(book_names[0].to_string());
        }
    }

    None
}

/// Get the official Hungarian book abbreviation
pub fn get_hungarian_book_name(book_id: Option<i32>) -> Option<String> {
    get_hungarian_abbrev(book_id).map(|s| s.to_string())
}
