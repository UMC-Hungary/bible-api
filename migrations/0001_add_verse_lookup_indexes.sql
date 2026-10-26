-- Without these, every verse lookup scans both tables (~125k rows read per
-- query), which exhausts the D1 free tier's 5M daily rows-read limit after
-- about 40 lookups.
CREATE INDEX IF NOT EXISTS idx_bible_verses_book_chapter_verse
    ON bible_verses (bible_books_id, chapter, verse);

CREATE INDEX IF NOT EXISTS idx_bible_verse_translation_verse_trans
    ON bible_verse_translation (bible_verses_id, trans);
