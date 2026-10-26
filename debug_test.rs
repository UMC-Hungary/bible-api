use rust_api::parser::parser::smart_parse_reference;

fn main() {
    let input = "mt5,1-2;5-6";
    let results = smart_parse_reference(input);
    
    println!("Input: {}", input);
    println!("Number of results: {}", results.len());
    
    for (i, result) in results.iter().enumerate() {
        println!("Result {}: label='{}', parsed_refs.len()={}", 
                 i, result.label, result.parsed_refs.len());
        for (j, parsed_ref) in result.parsed_refs.iter().enumerate() {
            println!("  Ref {}: book='{}', ch={:?}, v_from={:?}, v_to={:?}", 
                     j, parsed_ref.book, parsed_ref.chapter_from, 
                     parsed_ref.verse_from, parsed_ref.verse_to);
        }
    }
}
