/// Centralized book configuration with all name variations
/// This is the single source of truth for book names, IDs, and chapter counts
pub struct BookConfig {
    pub id: i32,
    pub names: Vec<&'static str>,
    pub chapters: u32,
}

pub fn get_book_configs() -> Vec<BookConfig> {
    vec![
        // Old Testament
        BookConfig {
            id: 1,
            names: vec!["gen", "genesis", "1móz", "1moz", "teremtés", "teremtes"],
            chapters: 50,
        },
        BookConfig {
            id: 2,
            names: vec![
                "ex",
                "exod",
                "exodus",
                "2móz",
                "2moz",
                "kivonulás",
                "kivonulas",
            ],
            chapters: 40,
        },
        BookConfig {
            id: 3,
            names: vec!["lev", "leviticus", "3móz", "3moz", "leviták", "levitak"],
            chapters: 27,
        },
        BookConfig {
            id: 4,
            names: vec!["num", "numbers", "4móz", "4moz", "számok", "szamok"],
            chapters: 36,
        },
        BookConfig {
            id: 5,
            names: vec![
                "deut",
                "deuteronomy",
                "5móz",
                "5moz",
                "törvények",
                "torvenyek",
            ],
            chapters: 34,
        },
        BookConfig {
            id: 6,
            names: vec!["josh", "joshua", "józs", "jozs", "józsué", "jozsue"],
            chapters: 24,
        },
        BookConfig {
            id: 7,
            names: vec!["judg", "judges", "bír", "bir", "bírák", "birak"],
            chapters: 21,
        },
        BookConfig {
            id: 8,
            names: vec!["ruth", "rut", "rút"],
            chapters: 4,
        },
        BookConfig {
            id: 9,
            names: vec!["1sam", "1samuel", "sám1", "sam1", "1sámuel", "1samuel"],
            chapters: 31,
        },
        BookConfig {
            id: 10,
            names: vec!["2sam", "2samuel", "sám2", "sam2", "2sámuel", "2samuel"],
            chapters: 24,
        },
        BookConfig {
            id: 11,
            names: vec!["1kgs", "1kings", "kir1", "1kir", "1királyok", "1kiralyok"],
            chapters: 22,
        },
        BookConfig {
            id: 12,
            names: vec!["2kgs", "2kings", "kir2", "2kir", "2királyok", "2kiralyok"],
            chapters: 25,
        },
        BookConfig {
            id: 13,
            names: vec![
                "1chr",
                "1chronicles",
                "krón1",
                "kron1",
                "1krónika",
                "1kronika",
            ],
            chapters: 29,
        },
        BookConfig {
            id: 14,
            names: vec![
                "2chr",
                "2chronicles",
                "krón2",
                "kron2",
                "2krónika",
                "2kronika",
            ],
            chapters: 36,
        },
        BookConfig {
            id: 15,
            names: vec!["ezra", "ezd", "ezsdrás", "ezsdras"],
            chapters: 10,
        },
        BookConfig {
            id: 16,
            names: vec!["neh", "nehemiah", "neh", "nehémiás", "nehemias"],
            chapters: 13,
        },
        BookConfig {
            id: 17,
            names: vec!["est", "esther", "eszt", "eszter"],
            chapters: 10,
        },
        BookConfig {
            id: 18,
            names: vec!["job", "jób", "job"],
            chapters: 42,
        },
        BookConfig {
            id: 19,
            names: vec![
                "ps",
                "psalm",
                "psalms",
                "zsolt",
                "zsoltár",
                "zsoltárok",
                "zsoltar",
                "zsoltarok",
            ],
            chapters: 150,
        },
        BookConfig {
            id: 20,
            names: vec![
                "prov",
                "proverbs",
                "péld",
                "peld",
                "példabeszédek",
                "peldabeszedek",
            ],
            chapters: 31,
        },
        BookConfig {
            id: 21,
            names: vec![
                "eccl",
                "ecclesiastes",
                "préd",
                "pred",
                "prédikátor",
                "predikator",
            ],
            chapters: 12,
        },
        BookConfig {
            id: 22,
            names: vec!["song", "songs", "én", "en", "énekek", "enekek"],
            chapters: 8,
        },
        BookConfig {
            id: 23,
            names: vec!["isa", "isaiah", "ézs", "ezs", "ézsaiás", "ezsaias"],
            chapters: 66,
        },
        BookConfig {
            id: 24,
            names: vec!["jer", "jeremiah", "jer", "jeremiás", "jeremias"],
            chapters: 52,
        },
        BookConfig {
            id: 25,
            names: vec!["lam", "lamentations", "siral", "siralmak"],
            chapters: 5,
        },
        BookConfig {
            id: 26,
            names: vec!["ezek", "ezekiel", "ez", "ezékiel"],
            chapters: 48,
        },
        BookConfig {
            id: 27,
            names: vec!["dan", "daniel", "dán", "dániel"],
            chapters: 12,
        },
        BookConfig {
            id: 28,
            names: vec!["hos", "hosea", "hós", "hóseás", "hoseas"],
            chapters: 14,
        },
        BookConfig {
            id: 29,
            names: vec!["joel", "jóel", "joel"],
            chapters: 3,
        },
        BookConfig {
            id: 30,
            names: vec!["amos", "ámós"],
            chapters: 9,
        },
        BookConfig {
            id: 31,
            names: vec!["obad", "obadiah", "abd", "abdiás", "abdias"],
            chapters: 1,
        },
        BookConfig {
            id: 32,
            names: vec!["jon", "jonah", "jón", "jónás", "jonas"],
            chapters: 4,
        },
        BookConfig {
            id: 33,
            names: vec!["mic", "micah", "mik", "mikeás", "mikeas"],
            chapters: 7,
        },
        BookConfig {
            id: 34,
            names: vec!["nah", "nahum", "náh", "náhum"],
            chapters: 3,
        },
        BookConfig {
            id: 35,
            names: vec!["hab", "habakkuk", "hab", "habakuk"],
            chapters: 3,
        },
        BookConfig {
            id: 36,
            names: vec!["zeph", "zephaniah", "zof", "zofóniás", "zofonias"],
            chapters: 3,
        },
        BookConfig {
            id: 37,
            names: vec!["hag", "haggai", "ag", "aggeus"],
            chapters: 2,
        },
        BookConfig {
            id: 38,
            names: vec!["zech", "zechariah", "zak", "zakariás", "zakarias"],
            chapters: 14,
        },
        BookConfig {
            id: 39,
            names: vec!["mal", "malachi", "mal", "malakiás", "malakias"],
            chapters: 4,
        },
        // New Testament
        BookConfig {
            id: 40,
            names: vec!["mt", "matt", "matthew", "mát", "mat", "máté", "mate"],
            chapters: 28,
        },
        BookConfig {
            id: 41,
            names: vec!["mk", "mark", "márk"],
            chapters: 16,
        },
        BookConfig {
            id: 42,
            names: vec!["lk", "luke", "luk", "lukács", "lukacs"],
            chapters: 24,
        },
        BookConfig {
            id: 43,
            names: vec!["jn", "john", "ján", "jan", "jános", "janos"],
            chapters: 21,
        },
        BookConfig {
            id: 44,
            names: vec!["acts", "csel", "cselekedetek"],
            chapters: 28,
        },
        BookConfig {
            id: 45,
            names: vec!["rom", "romans", "róm", "rom", "rómaiakhoz", "romaiakhoz"],
            chapters: 16,
        },
        BookConfig {
            id: 46,
            names: vec!["1cor", "1corinthians", "1kor", "kor1"],
            chapters: 16,
        },
        BookConfig {
            id: 47,
            names: vec!["2cor", "2corinthians", "2kor", "kor2"],
            chapters: 13,
        },
        BookConfig {
            id: 48,
            names: vec!["gal", "galatians", "gal", "galátákhoz", "galatakhoz"],
            chapters: 6,
        },
        BookConfig {
            id: 49,
            names: vec!["eph", "ephesians", "ef", "efézusiakhoz", "efezusiakhoz"],
            chapters: 6,
        },
        BookConfig {
            id: 50,
            names: vec!["phil", "philippians", "fil", "filippiekhez"],
            chapters: 4,
        },
        BookConfig {
            id: 51,
            names: vec!["col", "colossians", "kol", "kolosséiakhoz", "kolosseiakhoz"],
            chapters: 4,
        },
        BookConfig {
            id: 52,
            names: vec![
                "1thess",
                "1thessalonians",
                "1thessz",
                "thessz1",
                "1tesszalonika",
            ],
            chapters: 5,
        },
        BookConfig {
            id: 53,
            names: vec![
                "2thess",
                "2thessalonians",
                "2thessz",
                "thessz2",
                "2tesszalonika",
            ],
            chapters: 3,
        },
        BookConfig {
            id: 54,
            names: vec!["1tim", "1timothy", "tim1", "1timóteus", "1timoteus"],
            chapters: 6,
        },
        BookConfig {
            id: 55,
            names: vec!["2tim", "2timothy", "tim2", "2timóteus", "2timoteus"],
            chapters: 4,
        },
        BookConfig {
            id: 56,
            names: vec!["titus", "tit", "titusz"],
            chapters: 3,
        },
        BookConfig {
            id: 57,
            names: vec!["phlm", "philemon", "filem", "filemon", "filemonhoz"],
            chapters: 1,
        },
        BookConfig {
            id: 58,
            names: vec!["heb", "hebrews", "zsid", "zsidókhoz", "zsidokhoz"],
            chapters: 13,
        },
        BookConfig {
            id: 59,
            names: vec!["jas", "james", "jak", "jakab"],
            chapters: 5,
        },
        BookConfig {
            id: 60,
            names: vec!["1pet", "1peter", "pet1", "1pét", "pét1", "1péter"],
            chapters: 5,
        },
        BookConfig {
            id: 61,
            names: vec!["2pet", "2peter", "pet2", "2pét", "pét2", "2péter"],
            chapters: 3,
        },
        BookConfig {
            id: 62,
            names: vec![
                "1jn", "1john", "jn1", "1ján", "1jan", "ján1", "jan1", "1jános", "1janos",
            ],
            chapters: 5,
        },
        BookConfig {
            id: 63,
            names: vec![
                "2jn", "2john", "jn2", "2ján", "2jan", "ján2", "jan2", "2jános", "2janos",
            ],
            chapters: 1,
        },
        BookConfig {
            id: 64,
            names: vec![
                "3jn", "3john", "jn3", "3ján", "3jan", "ján3", "jan3", "3jános", "3janos",
            ],
            chapters: 1,
        },
        BookConfig {
            id: 65,
            names: vec!["jude", "júd", "júdás", "judas"],
            chapters: 1,
        },
        BookConfig {
            id: 66,
            names: vec![
                "rev",
                "revelation",
                "jel",
                "jelenések",
                "jelenesek",
                "apokalipszis",
            ],
            chapters: 22,
        },
    ]
}
