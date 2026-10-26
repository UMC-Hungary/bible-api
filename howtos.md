# Basic reference
curl "http://localhost:8787/suggest?term=gen%201:1" | jq '.'

# Multiple references
curl "http://localhost:8787/suggest?term=1Kor%202,2-3;Jn%201,1-2" | jq '.'

# Contextual references (same book, same chapter)
curl "http://localhost:8787/suggest?term=mt5,1-2;5-6" | jq '.'

# Book only
curl "http://localhost:8787/suggest?term=genesis" | jq '.'

# With translation suffix
curl "http://localhost:8787/suggest?term=1Kor%202,2-3;Jn%201,1-2/KG" | jq '.'

# Error case (missing term)
curl "http://localhost:8787/suggest"

curl -s "http://localhost:8787/suggest?term=john%203:16-17" | jq '.'


Start server
`npx wrangler dev --port 8787`


Get data from d1 db:
npx wrangler d1 execute SEARCH_DB --local --command "
  SELECT v.verse, t.text, t.trans
  FROM bible_verses v
  JOIN bible_verse_translation t ON t.bible_verses_id = v.id
  WHERE v.bible_books_id = 42 AND v.chapter = 9
  ORDER BY v.verse, t.trans;
"
