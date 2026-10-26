use crate::types::*;

pub fn create_error_response(error_msg: &str) -> ErrorResponse {
    ErrorResponse {
        error: error_msg.to_string(),
    }
}

pub fn create_search_suggestion(
    hit_count: u32,
    category: &str,
    label: &str,
    link: &str,
) -> SearchSuggestion {
    SearchSuggestion {
        hit_count,
        cat: category.to_string(),
        label: label.to_string(),
        link: link.to_string(),
        parsed_refs: None,
    }
}

pub fn create_search_suggestion_with_refs(
    hit_count: u32,
    category: &str,
    label: &str,
    link: &str,
    parsed_refs: Vec<ParsedReference>,
) -> SearchSuggestion {
    SearchSuggestion {
        hit_count,
        cat: category.to_string(),
        label: label.to_string(),
        link: link.to_string(),
        parsed_refs: Some(parsed_refs),
    }
}

pub fn create_suggestions_response(
    suggestions: Vec<SearchSuggestion>,
) -> SearchSuggestionsResponse {
    let total_count = suggestions.iter().map(|s| s.hit_count).sum();
    SearchSuggestionsResponse {
        suggestions,
        total_count,
    }
}
