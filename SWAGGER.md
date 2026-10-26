# 📖 API Documentation - Swagger/OpenAPI

This directory contains the OpenAPI/Swagger documentation for the Bible API.

## 📄 Files

- **`openapi.yaml`** - Complete OpenAPI 3.0 specification (source of truth)
- **`api-docs.html`** - Generated Redoc documentation (created via `make docs`, gitignored)
- **`SWAGGER.md`** - This file

## 🚀 Quick Start

### Live Swagger UI

The API serves the generated Redoc documentation directly at the `/swagger` endpoint:

**Production:** `https://api.nyiregyhazimetodista.hu/swagger`
**Local Dev:** `http://localhost:8787/swagger`

**Note:** The documentation is generated from `openapi.yaml` using Redoc. To regenerate:
```bash
make docs  # Generates api-docs.html
make build # Includes it in the worker
```

You can also access the raw OpenAPI spec at:
- `https://api.nyiregyhazimetodista.hu/openapi.yaml`

### Alternative Options

### Option 1: Generate Beautiful Redoc Documentation

Generate a standalone, production-ready documentation page using Redocly:

```bash
# Generate documentation (requires Node.js)
make docs

# Open the generated file
open api-docs.html
```

The generated `api-docs.html` is a self-contained file with beautiful, responsive documentation.

### Option 2: Online Swagger Editor

Visit [Swagger Editor](https://editor.swagger.io/) and:
1. Click **File → Import File**
2. Select the `openapi.yaml` file
3. View and test the API interactively

### Test the API

Once you open the API documentation (Redoc or Swagger Editor), you can:

1. **Explore Endpoints**: Click on any endpoint to expand its details
2. **Try It Out**: Click the "Try it out" button
3. **Enter Parameters**: Fill in the required parameters (e.g., `term: gen 1:1`)
4. **Execute**: Click "Execute" to make a real API call
5. **View Response**: See the actual response from the API

## 📝 Example Requests

### Simple Verse Lookup
```bash
curl "https://api.nyiregyhazimetodista.hu/suggest/gen%201:1"
```

### With RUF Translation
```bash
curl "https://api.nyiregyhazimetodista.hu/suggest/gen%201:1/RUF"
curl "https://api.nyiregyhazimetodista.hu/suggest/mt1,1-2/RUF"
```

### Multiple References
```bash
curl "https://api.nyiregyhazimetodista.hu/suggest/1Kor%202,2-3;Jn%201,1-2"
```

### Contextual References
```bash
curl "https://api.nyiregyhazimetodista.hu/suggest/mt5,1-2;5-6"
curl "https://api.nyiregyhazimetodista.hu/suggest/mt5,1-2;5-6/UF"
```

## 🔧 Using the OpenAPI Spec

### Generate API Clients

You can generate client libraries in multiple languages using the OpenAPI spec:

```bash
# Install OpenAPI Generator
npm install @openapitools/openapi-generator-cli -g

# Generate TypeScript client
openapi-generator-cli generate -i openapi.yaml -g typescript-fetch -o ./clients/typescript

# Generate Python client
openapi-generator-cli generate -i openapi.yaml -g python -o ./clients/python

# Generate Java client
openapi-generator-cli generate -i openapi.yaml -g java -o ./clients/java

# Generate Go client
openapi-generator-cli generate -i openapi.yaml -g go -o ./clients/go
```

### Validate the Spec

```bash
# Using OpenAPI CLI
npm install -g @apidevtools/swagger-cli
swagger-cli validate openapi.yaml

# Using Redocly CLI
npm install -g @redocly/cli
redocly lint openapi.yaml
```

### Generate Alternative Documentation

#### Using Redoc

Create a beautiful single-page documentation:

```bash
# Install Redoc CLI
npm install -g redoc-cli

# Generate standalone HTML
redoc-cli bundle openapi.yaml -o api-docs.html
```

#### Using Postman

1. Import `openapi.yaml` into Postman
2. Create a new Collection from the spec
3. Test all endpoints directly in Postman

## 📚 API Reference

### Endpoint: GET /suggest

**Description**: Search and parse Bible references

**Parameters (in order)**:
1. `term` (required, string): Bible reference to parse (e.g., "gen 1:1")
2. `translation` (optional, string, default: "UF"): Translation code (UF=Újfordítású, RUF=Revised Újfordítású)

**Response**: `SimplifiedResponse` object containing:
- `label` (string): Normalized reference label
- `link` (string): URL-friendly link
- `parsed_refs` (array): Parsed reference components
- `verses` (array): Verse objects with chapter, verse, and text
- `verses_as_text` (array): Combined verse texts

### Supported Reference Formats

- **Simple verse**: `gen 1:1`, `mt 5:3`
- **Verse range**: `jn 3:16-17`, `rom 8:28-30`
- **Multiple refs**: `1Kor 2,2-3;Jn 1,1-2`
- **Contextual**: `mt5,1-2;5-6` (same chapter)
- **Cross-chapter**: `mt5,1-2;6,5-6` (different chapters)
- **With translation**: `gen 1:1/en`, `mt5/KG`

### Supported Books

**Old Testament**: Genesis (gen), Exodus (ex), Leviticus (lev), Numbers (num), Deuteronomy (deut), Joshua (josh), Judges (judg), Ruth, 1 Samuel (1sam), 2 Samuel (2sam), 1 Kings (1kgs), 2 Kings (2kgs), 1 Chronicles (1chr), 2 Chronicles (2chr), Ezra, Nehemiah (neh), Esther (est), Job, Psalms (ps), Proverbs (prov), Ecclesiastes (eccl), Song of Solomon (song), Isaiah (isa), Jeremiah (jer), Lamentations (lam), Ezekiel (ezek), Daniel (dan), Hosea (hos), Joel, Amos, Obadiah (obad), Jonah (jon), Micah (mic), Nahum (nah), Habakkuk (hab), Zephaniah (zeph), Haggai (hag), Zechariah (zech), Malachi (mal)

**New Testament**: Matthew (mt), Mark (mk), Luke (lk), John (jn), Acts, Romans (rom), 1 Corinthians (1cor/1kor), 2 Corinthians (2cor/2kor), Galatians (gal), Ephesians (eph), Philippians (phil), Colossians (col), 1 Thessalonians (1thess), 2 Thessalonians (2thess), 1 Timothy (1tim), 2 Timothy (2tim), Titus, Philemon (phlm), Hebrews (heb), James (jas), 1 Peter (1pet), 2 Peter (2pet), 1 John (1jn), 2 John (2jn), 3 John (3jn), Jude, Revelation (rev)

## 🔗 Additional Tools

### Swagger Tools
- [Swagger Editor](https://editor.swagger.io/) - Online editor and validator
- [Swagger UI](https://swagger.io/tools/swagger-ui/) - Interactive API documentation
- [Swagger Codegen](https://swagger.io/tools/swagger-codegen/) - Generate client/server code

### OpenAPI Tools
- [OpenAPI Generator](https://openapi-generator.tech/) - Generate clients, servers, and docs
- [Redoc](https://redocly.com/redoc/) - Beautiful API documentation
- [Postman](https://www.postman.com/) - API testing and documentation

### Validation & Testing
- [Swagger CLI](https://github.com/APIDevTools/swagger-cli) - Command-line validation
- [Spectral](https://stoplight.io/open-source/spectral) - OpenAPI linting
- [Prism](https://stoplight.io/open-source/prism) - Mock API server

## 📞 Support

For issues or questions:
- Check the main [README.md](./README.md)
- Review the OpenAPI spec: `openapi.yaml`
- Test endpoints using Swagger UI: `swagger-ui.html`

---

*Generated for the Bible API - Rust Cloudflare Worker*
