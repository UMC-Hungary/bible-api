/// Hungarian book name mappings
/// Maps book IDs to their official Hungarian abbreviations
pub fn get_hungarian_abbrev(book_id: Option<i32>) -> Option<&'static str> {
    let book_id = book_id?;

    match book_id {
        // Old Testament
        1 => Some("1Móz"),   // Genesis
        2 => Some("2Móz"),   // Exodus
        3 => Some("3Móz"),   // Leviticus
        4 => Some("4Móz"),   // Numbers
        5 => Some("5Móz"),   // Deuteronomy
        6 => Some("Józs"),   // Joshua
        7 => Some("Bír"),    // Judges
        8 => Some("Rut"),    // Ruth
        9 => Some("1Sám"),   // 1 Samuel
        10 => Some("2Sám"),  // 2 Samuel
        11 => Some("1Kir"),  // 1 Kings
        12 => Some("2Kir"),  // 2 Kings
        13 => Some("1Krón"), // 1 Chronicles
        14 => Some("2Krón"), // 2 Chronicles
        15 => Some("Ezsd"),  // Ezra
        16 => Some("Neh"),   // Nehemiah
        17 => Some("Eszt"),  // Esther
        18 => Some("Jób"),   // Job
        19 => Some("Zsolt"), // Psalms
        20 => Some("Péld"),  // Proverbs
        21 => Some("Préd"),  // Ecclesiastes
        22 => Some("Én"),    // Song of Solomon
        23 => Some("Ézs"),   // Isaiah
        24 => Some("Jer"),   // Jeremiah
        25 => Some("JSir"),  // Lamentations
        26 => Some("Ez"),    // Ezekiel
        27 => Some("Dán"),   // Daniel
        28 => Some("Hós"),   // Hosea
        29 => Some("Jóel"),  // Joel
        30 => Some("Am"),    // Amos
        31 => Some("Abd"),   // Obadiah
        32 => Some("Jón"),   // Jonah
        33 => Some("Mik"),   // Micah
        34 => Some("Náh"),   // Nahum
        35 => Some("Hab"),   // Habakkuk
        36 => Some("Zof"),   // Zephaniah
        37 => Some("Hag"),   // Haggai
        38 => Some("Zak"),   // Zechariah
        39 => Some("Mal"),   // Malachi

        // New Testament
        40 => Some("Mt"),     // Matthew
        41 => Some("Mk"),     // Mark
        42 => Some("Lk"),     // Luke
        43 => Some("Jn"),     // John
        44 => Some("ApCsel"), // Acts
        45 => Some("Róm"),    // Romans
        46 => Some("1Kor"),   // 1 Corinthians
        47 => Some("2Kor"),   // 2 Corinthians
        48 => Some("Gal"),    // Galatians
        49 => Some("Ef"),     // Ephesians
        50 => Some("Fil"),    // Philippians
        51 => Some("Kol"),    // Colossians
        52 => Some("1Tesz"),  // 1 Thessalonians
        53 => Some("2Tesz"),  // 2 Thessalonians
        54 => Some("1Tim"),   // 1 Timothy
        55 => Some("2Tim"),   // 2 Timothy
        56 => Some("Tit"),    // Titus
        57 => Some("Filem"),  // Philemon
        58 => Some("Zsid"),   // Hebrews
        59 => Some("Jak"),    // James
        60 => Some("1Pt"),    // 1 Peter
        61 => Some("2Pt"),    // 2 Peter
        62 => Some("1Jn"),    // 1 John
        63 => Some("2Jn"),    // 2 John
        64 => Some("3Jn"),    // 3 John
        65 => Some("Júd"),    // Jude
        66 => Some("Jel"),    // Revelation

        _ => None,
    }
}
