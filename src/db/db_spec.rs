// Tests for database module

use super::*;

#[test]
fn test_get_book_id_old_testament() {
    // Test Old Testament books
    assert_eq!(get_book_id_from_name("gen"), Some(1));
    assert_eq!(get_book_id_from_name("genesis"), Some(1));
    assert_eq!(get_book_id_from_name("ex"), Some(2));
    assert_eq!(get_book_id_from_name("exodus"), Some(2));
    assert_eq!(get_book_id_from_name("ps"), Some(19));
    assert_eq!(get_book_id_from_name("psalms"), Some(19));
    assert_eq!(get_book_id_from_name("mal"), Some(39));
    assert_eq!(get_book_id_from_name("malachi"), Some(39));
}

#[test]
fn test_get_book_id_new_testament() {
    // Test New Testament books
    assert_eq!(get_book_id_from_name("mt"), Some(40));
    assert_eq!(get_book_id_from_name("matthew"), Some(40));
    assert_eq!(get_book_id_from_name("jn"), Some(43));
    assert_eq!(get_book_id_from_name("john"), Some(43));
    assert_eq!(get_book_id_from_name("1cor"), Some(46));
    assert_eq!(get_book_id_from_name("1kor"), Some(46));
    assert_eq!(get_book_id_from_name("rev"), Some(66));
    assert_eq!(get_book_id_from_name("revelation"), Some(66));
}

#[test]
fn test_get_book_id_case_insensitive() {
    // Test case insensitivity
    assert_eq!(get_book_id_from_name("GEN"), Some(1));
    assert_eq!(get_book_id_from_name("Genesis"), Some(1));
    assert_eq!(get_book_id_from_name("MATTHEW"), Some(40));
    assert_eq!(get_book_id_from_name("Mt"), Some(40));
}

#[test]
fn test_get_book_id_invalid() {
    // Test invalid book names
    assert_eq!(get_book_id_from_name("invalid"), None);
    assert_eq!(get_book_id_from_name(""), None);
    assert_eq!(get_book_id_from_name("xyz"), None);
    assert_eq!(get_book_id_from_name("123"), None);
}

#[test]
fn test_get_book_id_numbered_books() {
    // Test numbered books (Samuel, Kings, Chronicles, etc.)
    assert_eq!(get_book_id_from_name("1sam"), Some(9));
    assert_eq!(get_book_id_from_name("2sam"), Some(10));
    assert_eq!(get_book_id_from_name("1kgs"), Some(11));
    assert_eq!(get_book_id_from_name("2kgs"), Some(12));
    assert_eq!(get_book_id_from_name("1chr"), Some(13));
    assert_eq!(get_book_id_from_name("2chr"), Some(14));

    // NT numbered books
    assert_eq!(get_book_id_from_name("1tim"), Some(54));
    assert_eq!(get_book_id_from_name("2tim"), Some(55));
    assert_eq!(get_book_id_from_name("1pet"), Some(60));
    assert_eq!(get_book_id_from_name("2pet"), Some(61));
    assert_eq!(get_book_id_from_name("1jn"), Some(62));
    assert_eq!(get_book_id_from_name("2jn"), Some(63));
    assert_eq!(get_book_id_from_name("3jn"), Some(64));
}

#[test]
fn test_get_book_id_hungarian_names() {
    // Test Hungarian book names
    assert_eq!(get_book_id_from_name("1móz"), Some(1));
    assert_eq!(get_book_id_from_name("teremtés"), Some(1));
    assert_eq!(get_book_id_from_name("zsolt"), Some(19));
    assert_eq!(get_book_id_from_name("zsoltárok"), Some(19));
    assert_eq!(get_book_id_from_name("mát"), Some(40));
    assert_eq!(get_book_id_from_name("máté"), Some(40));
    assert_eq!(get_book_id_from_name("ján"), Some(43));
    assert_eq!(get_book_id_from_name("jános"), Some(43));
    assert_eq!(get_book_id_from_name("1kor"), Some(46));
    assert_eq!(get_book_id_from_name("róm"), Some(45));
    assert_eq!(get_book_id_from_name("jel"), Some(66));
    assert_eq!(get_book_id_from_name("jelenések"), Some(66));
}

#[test]
fn test_get_book_id_hungarian_without_accents() {
    // Test Hungarian names without accents (common typing)
    assert_eq!(get_book_id_from_name("1moz"), Some(1));
    assert_eq!(get_book_id_from_name("teremtes"), Some(1));
    assert_eq!(get_book_id_from_name("zsoltarok"), Some(19));
    assert_eq!(get_book_id_from_name("mat"), Some(40));
    assert_eq!(get_book_id_from_name("mate"), Some(40));
    assert_eq!(get_book_id_from_name("jan"), Some(43));
    assert_eq!(get_book_id_from_name("janos"), Some(43));
    assert_eq!(get_book_id_from_name("rom"), Some(45));
    assert_eq!(get_book_id_from_name("jelenesek"), Some(66));
}

#[test]
fn test_get_book_id_all_66_books() {
    // Comprehensive test for all 66 Bible books
    let books = vec![
        ("gen", 1),
        ("ex", 2),
        ("lev", 3),
        ("num", 4),
        ("deut", 5),
        ("josh", 6),
        ("judg", 7),
        ("ruth", 8),
        ("1sam", 9),
        ("2sam", 10),
        ("1kgs", 11),
        ("2kgs", 12),
        ("1chr", 13),
        ("2chr", 14),
        ("ezra", 15),
        ("neh", 16),
        ("est", 17),
        ("job", 18),
        ("ps", 19),
        ("prov", 20),
        ("eccl", 21),
        ("song", 22),
        ("isa", 23),
        ("jer", 24),
        ("lam", 25),
        ("ezek", 26),
        ("dan", 27),
        ("hos", 28),
        ("joel", 29),
        ("amos", 30),
        ("obad", 31),
        ("jon", 32),
        ("mic", 33),
        ("nah", 34),
        ("hab", 35),
        ("zeph", 36),
        ("hag", 37),
        ("zech", 38),
        ("mal", 39),
        // New Testament
        ("mt", 40),
        ("mk", 41),
        ("lk", 42),
        ("jn", 43),
        ("acts", 44),
        ("rom", 45),
        ("1cor", 46),
        ("2cor", 47),
        ("gal", 48),
        ("eph", 49),
        ("phil", 50),
        ("col", 51),
        ("1thess", 52),
        ("2thess", 53),
        ("1tim", 54),
        ("2tim", 55),
        ("titus", 56),
        ("phlm", 57),
        ("heb", 58),
        ("jas", 59),
        ("1pet", 60),
        ("2pet", 61),
        ("1jn", 62),
        ("2jn", 63),
        ("3jn", 64),
        ("jude", 65),
        ("rev", 66),
    ];

    for (book_name, expected_id) in books {
        assert_eq!(
            get_book_id_from_name(book_name),
            Some(expected_id),
            "Failed for book: {}",
            book_name
        );
    }
}

#[test]
fn test_get_trans_id_uf() {
    // Test UF translation mapping
    assert_eq!(get_trans_id("UF"), 0);
    assert_eq!(get_trans_id("uf"), 0);
    assert_eq!(get_trans_id("Uf"), 0);
}

#[test]
fn test_get_trans_id_ruf() {
    // Test RUF translation mapping
    assert_eq!(get_trans_id("RUF"), 1);
    assert_eq!(get_trans_id("ruf"), 1);
    assert_eq!(get_trans_id("Ruf"), 1);
}

#[test]
fn test_get_trans_id_default() {
    // Test default translation (should default to UF = 0)
    assert_eq!(get_trans_id(""), 0);
    assert_eq!(get_trans_id("invalid"), 0);
    assert_eq!(get_trans_id("KG"), 0);
    assert_eq!(get_trans_id("ESV"), 0);
}
