use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Serialize, Debug)]
pub struct SearchSuggestion {
    pub hit_count: u32,
    pub cat: String,
    pub label: String,
    pub link: String,
    pub parsed_refs: Option<Vec<ParsedReference>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ParsedReference {
    pub book: String,
    pub book_id: Option<i32>,
    pub chapter_from: Option<u32>,
    pub chapter_to: Option<u32>,
    pub verse_from: Option<u32>,
    pub verse_to: Option<u32>,
}

#[derive(Serialize, Debug)]
pub struct SearchSuggestionsResponse {
    pub suggestions: Vec<SearchSuggestion>,
    pub total_count: u32,
}

#[derive(Serialize, Debug)]
pub struct VerseResponse {
    pub chapter: i32,
    pub verse: i32,
    pub text: String,
}

#[derive(Serialize, Debug)]
pub struct SimplifiedResponse {
    pub label: String,
    pub hungarian_label: String, // Hungarian translation of the reference label
    pub link: String,
    pub parsed_refs: Vec<ParsedReference>,
    pub verses: Vec<VerseResponse>, // Array of verse objects with chapter, verse, text
    pub verses_as_text: Vec<String>, // Combined text as before
}

#[derive(Serialize, Debug)]
pub struct BibleVerse {
    pub id: i32,
    pub bible_books_id: i32,
    pub chapter: i32,
    pub verse: i32,
    pub code: String,
    pub code_string: String,
}

#[derive(Serialize, Debug)]
pub struct BibleBook {
    pub id: i32,
    pub name: String,
    pub short_name: String,
    pub number: i32,
    pub search_name: String,
    pub abbrev: String,
}

#[derive(Serialize, Debug)]
pub struct BibleVerseTranslation {
    pub id: i32,
    pub bible_verses_id: i32,
    pub text: String,
    pub trans: i32,
}

#[derive(Serialize, Debug)]
pub struct VerseWithTranslation {
    pub verse: BibleVerse,
    pub book: BibleBook,
    pub translation: Option<BibleVerseTranslation>,
}

#[derive(Serialize, Debug)]
pub struct VerseSearchResponse {
    pub query: String,
    pub translation: String,
    pub verses: Vec<VerseWithTranslation>,
    pub found: bool,
}
