# 📖 Bible API - Quick Reference

## 🚀 Base URLs

- **Production**: `https://api.nyiregyhazimetodista.hu`
- **Local Dev**: `http://localhost:8787`

## 🔗 Endpoints

### GET /swagger

Interactive Swagger UI documentation for the API.

**URL:** `https://api.nyiregyhazimetodista.hu/swagger`

### GET /openapi.yaml

Raw OpenAPI specification in YAML format.

**URL:** `https://api.nyiregyhazimetodista.hu/openapi.yaml`

### GET /suggest

Parse Bible references and retrieve verse content.

#### Request

```http
GET /suggest/{term}
GET /suggest/{term}/{translation}
```

**Parameters (in order):**

1. **term** (required) - Bible reference to parse
2. **translation** (optional) - Translation code: `UF` or `RUF`

**Path Parameters:**
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `term` | string | ✅ Yes | - | Bible reference to parse (first parameter) |
| `translation` | string | ❌ No | `UF` | Translation code (UF=trans=0, RUF=trans=1) (second parameter) |

#### Response: 200 OK

```json
{
  "label": "string",
  "link": "string",
  "parsed_refs": [
    {
      "book": "string",
      "book_id": 1,
      "chapter_from": 1,
      "chapter_to": 1,
      "verse_from": 1,
      "verse_to": 1
    }
  ],
  "verses": [
    {
      "chapter": 1,
      "verse": 1,
      "text": "string"
    }
  ],
  "verses_as_text": ["string"]
}
```

#### Response: 400 Bad Request

```json
{
  "error": "Missing search term"
}
```

## 📝 Reference Format Examples

### Basic Formats

| Format | Example | Description |
|--------|---------|-------------|
| Book only | `gen`, `matthew` | Entire book |
| Book + Chapter | `gen 1`, `mt5` | Entire chapter |
| Book + Chapter:Verse | `gen 1:1`, `mt 5:3` | Single verse |
| Verse Range | `jn 3:16-17` | Multiple verses |
| Chapter Range | `gen 1-3` | Multiple chapters |

### Advanced Formats

| Format | Example | Description |
|--------|---------|-------------|
| Multiple References | `1Kor 2,2-3;Jn 1,1-2` | Semicolon-separated |
| Contextual Same Chapter | `mt5,1-2;5-6` | Same chapter, different verses |
| Contextual Cross Chapter | `mt5,1-2;6,5-6` | Same book, different chapters |
| With Translation | `gen 1:1/en` | Specific translation |
| Complex Pattern | `1Kor 2,2-3.4` | Verses 2-3 and 4 |

## 📚 Supported Books

### Old Testament (39 books)

| Abbreviation | Full Name | Book ID |
|--------------|-----------|---------|
| `gen` | Genesis | 1 |
| `ex`, `exod` | Exodus | 2 |
| `lev` | Leviticus | 3 |
| `num` | Numbers | 4 |
| `deut` | Deuteronomy | 5 |
| `josh` | Joshua | 6 |
| `judg` | Judges | 7 |
| `ruth` | Ruth | 8 |
| `1sam` | 1 Samuel | 9 |
| `2sam` | 2 Samuel | 10 |
| `1kgs` | 1 Kings | 11 |
| `2kgs` | 2 Kings | 12 |
| `1chr` | 1 Chronicles | 13 |
| `2chr` | 2 Chronicles | 14 |
| `ezra` | Ezra | 15 |
| `neh` | Nehemiah | 16 |
| `est` | Esther | 17 |
| `job` | Job | 18 |
| `ps`, `psalm` | Psalms | 19 |
| `prov` | Proverbs | 20 |
| `eccl` | Ecclesiastes | 21 |
| `song` | Song of Solomon | 22 |
| `isa` | Isaiah | 23 |
| `jer` | Jeremiah | 24 |
| `lam` | Lamentations | 25 |
| `ezek` | Ezekiel | 26 |
| `dan` | Daniel | 27 |
| `hos` | Hosea | 28 |
| `joel` | Joel | 29 |
| `amos` | Amos | 30 |
| `obad` | Obadiah | 31 |
| `jon` | Jonah | 32 |
| `mic` | Micah | 33 |
| `nah` | Nahum | 34 |
| `hab` | Habakkuk | 35 |
| `zeph` | Zephaniah | 36 |
| `hag` | Haggai | 37 |
| `zech` | Zechariah | 38 |
| `mal` | Malachi | 39 |

### New Testament (27 books)

| Abbreviation | Full Name | Book ID |
|--------------|-----------|---------|
| `mt`, `matt` | Matthew | 40 |
| `mk` | Mark | 41 |
| `lk` | Luke | 42 |
| `jn`, `jhn` | John | 43 |
| `acts` | Acts | 44 |
| `rom` | Romans | 45 |
| `1cor`, `1kor` | 1 Corinthians | 46 |
| `2cor`, `2kor` | 2 Corinthians | 47 |
| `gal` | Galatians | 48 |
| `eph` | Ephesians | 49 |
| `phil` | Philippians | 50 |
| `col` | Colossians | 51 |
| `1thess` | 1 Thessalonians | 52 |
| `2thess` | 2 Thessalonians | 53 |
| `1tim` | 1 Timothy | 54 |
| `2tim` | 2 Timothy | 55 |
| `titus` | Titus | 56 |
| `phlm` | Philemon | 57 |
| `heb` | Hebrews | 58 |
| `jas` | James | 59 |
| `1pet` | 1 Peter | 60 |
| `2pet` | 2 Peter | 61 |
| `1jn` | 1 John | 62 |
| `2jn` | 2 John | 63 |
| `3jn` | 3 John | 64 |
| `jude` | Jude | 65 |
| `rev` | Revelation | 66 |

## 🧪 cURL Examples

### Simple Verse Lookup
```bash
curl "https://api.nyiregyhazimetodista.hu/suggest/gen%201:1"
```

### Multiple References
```bash
curl "https://api.nyiregyhazimetodista.hu/suggest/1Kor%202,2-3;Jn%201,1-2"
```

### Contextual References
```bash
curl "https://api.nyiregyhazimetodista.hu/suggest/mt5,1-2;5-6"
```

### With Specific Translation
```bash
curl "https://api.nyiregyhazimetodista.hu/suggest/jn%203:16/UF"
curl "https://api.nyiregyhazimetodista.hu/suggest/mt1,1-2/RUF"
```

## 🔧 Integration Examples

### JavaScript/TypeScript
```javascript
async function getBibleVerse(reference, translation = 'UF') {
  const url = `https://api.nyiregyhazimetodista.hu/suggest/${encodeURIComponent(reference)}/${translation}`;
  
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`API error: ${response.status}`);
  }
  
  return await response.json();
}

// Usage
const result = await getBibleVerse('gen 1:1');
console.log(result.verses_as_text[0]);
```

### Python
```python
import requests

def get_bible_verse(reference, translation='UF'):
    url = f'https://api.nyiregyhazimetodista.hu/suggest/{requests.utils.quote(reference)}/{translation}'
    
    response = requests.get(url)
    response.raise_for_status()
    
    return response.json()

# Usage
result = get_bible_verse('gen 1:1')
print(result['verses_as_text'][0])
```

### Go
```go
package main

import (
    "encoding/json"
    "fmt"
    "net/http"
    "net/url"
)

type SimplifiedResponse struct {
    Label        string   `json:"label"`
    Link         string   `json:"link"`
    VersesAsText []string `json:"verses_as_text"`
}

func getBibleVerse(reference, translation string) (*SimplifiedResponse, error) {
    baseURL := fmt.Sprintf("https://api.nyiregyhazimetodista.hu/suggest/%s/%s",
        url.PathEscape(reference), translation)
    
    resp, err := http.Get(baseURL)
    if err != nil {
        return nil, err
    }
    defer resp.Body.Close()
    
    var result SimplifiedResponse
    if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
        return nil, err
    }
    
    return &result, nil
}

// Usage
func main() {
    result, err := getBibleVerse("gen 1:1", "UF")
    if err != nil {
        panic(err)
    }
    fmt.Println(result.VersesAsText[0])
}
```

## 📊 Response Status Codes

| Code | Description | Example |
|------|-------------|---------|
| 200 | Success | Valid reference parsed and verses retrieved |
| 400 | Bad Request | Missing required parameter `term` |
| 500 | Server Error | Database connection error |

## 🌍 Translation Codes

Available Bible translations:
- `UF` - Újfordítású Biblia / New Translation (default, currently available)
- `RUF` - Revised Újfordítású Biblia (coming soon)

## 📖 Full Documentation

- **API Documentation**: Generate with `make docs` and open `api-docs.html`
- **OpenAPI Spec**: [`openapi.yaml`](./openapi.yaml)
- **Detailed Guide**: [`SWAGGER.md`](./SWAGGER.md)
- **Online Editor**: [Swagger Editor](https://editor.swagger.io/) - Import `openapi.yaml`

---

*For more details, see the complete [README.md](./README.md)*
