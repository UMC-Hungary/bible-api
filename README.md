# Bible API - Rust Cloudflare Worker

A high-performance Bible reference parsing and verse lookup API built with Rust and deployed on Cloudflare Workers. Features intelligent context-aware parsing of Bible references in multiple formats.

## 🚀 Features

- **⚡ Cloudflare Workers**: Runs on Cloudflare's global edge computing platform
- **🔧 WebAssembly**: Compiled to WASM for optimal performance (~1ms startup time)
- **🔍 GET /suggest**: Bible reference parsing endpoint with smart context-aware parsing
- **📖 OpenAPI/Swagger**: Complete API documentation with interactive testing interface
- **📄 JSON responses**: All responses are in JSON format with proper error handling
- **🧪 Comprehensive Testing**: 50+ unit tests with full integration test coverage
- **🌍 Edge deployment**: Global deployment with sub-millisecond response times
- **🔄 Local Development**: Hot-reload development server with auto-detection
- **📊 Code Coverage**: Multiple report formats (HTML, XML, JSON)
- **⚙️ Environment Config**: Flexible configuration with .env support

## 📖 API Documentation

### 🌐 Live Interactive Documentation

**Swagger UI** (Recommended): [`https://api.nyiregyhazimetodista.hu/swagger`](https://api.nyiregyhazimetodista.hu/swagger)
- Interactive API documentation
- Try out endpoints directly in your browser
- View request/response examples

**OpenAPI Spec**: [`https://api.nyiregyhazimetodista.hu/openapi.yaml`](https://api.nyiregyhazimetodista.hu/openapi.yaml)
- Raw OpenAPI 3.0 specification

### 📚 Additional Resources

**Quick Reference**: [`API_REFERENCE.md`](./API_REFERENCE.md) - Fast lookup of endpoints, parameters, and examples

**Swagger Guide**: [`SWAGGER.md`](./SWAGGER.md) - Complete guide for API documentation

**Postman Collection**: [`postman_collection.json`](./postman_collection.json) - Import into Postman for easy testing

### Alternative Documentation Options

**Option 1: Generate Beautiful Redoc Documentation**
```bash
# Generate static documentation (requires Node.js)
make docs

# Open generated docs
open api-docs.html
```

**Option 2: Online Swagger Editor**
Use the [Online Swagger Editor](https://editor.swagger.io/) and import `openapi.yaml`.

## API Endpoints

### GET /swagger

Interactive Swagger UI documentation.

**URL:** `https://api.nyiregyhazimetodista.hu/swagger`

### GET /openapi.yaml

Raw OpenAPI specification in YAML format.

**URL:** `https://api.nyiregyhazimetodista.hu/openapi.yaml`

### GET /suggest

Search and parse Bible references with intelligent context-aware parsing.

**Endpoints:**
- `GET /suggest/{term}` - Default UF translation
- `GET /suggest/{term}/{translation}` - Specify translation

**Parameters (in order):**
1. `term` (required, string): Bible reference to parse (e.g., "gen 1:1", "mt5,1-2;6,5-6")
2. `translation` (optional, string, default: "UF"): Translation code
   - `UF` - Újfordítású Biblia / New Translation (default, trans=0)
   - `RUF` - Revideált Új Fordítás / Revised New Translation (trans=1)

**Supported Reference Formats:**
- Simple verse: `gen 1:1`, `mt 5:3`
- Verse range: `jn 3:16-17`, `rom 8:28-30`
- Multiple references: `1Kor 2,2-3;Jn 1,1-2`
- Contextual: `mt5,1-2;5-6` (same chapter, different verses)
- Cross-chapter: `mt5,1-2;6,5-6` (same book, different chapters)
- With translation: `gen 1:1/UF`

**Example requests:**
```bash
# Simple verse lookup (uses default UF translation)
curl "https://api.nyiregyhazimetodista.hu/suggest/gen%201:1"

# Using RUF translation
curl "https://api.nyiregyhazimetodista.hu/suggest/gen%201:1/RUF"

# Multiple references
curl "https://api.nyiregyhazimetodista.hu/suggest/mt1,1-2/UF"

# Response
{
  "label": "gen 1:1",
  "link": "/gen 1:1",
  "parsed_refs": [
    {
      "book": "gen",
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
      "text": "In the beginning God created the heaven and the earth."
    }
  ],
  "verses_as_text": [
    "In the beginning God created the heaven and the earth."
  ]
}

# Multiple references with specific translation
curl "https://api.nyiregyhazimetodista.hu/suggest?term=1Kor%202,2-3;Jn%201,1-2&translation=UF"

# Missing parameter (400 Bad Request)
curl "https://api.nyiregyhazimetodista.hu/suggest"

# Error response
{
  "error": "Missing search term"
}
```

## 🛠️ Development

### Prerequisites

- **Rust** (latest stable version) - [Install Rust](https://rustup.rs/)
- **Node.js** and **npm** (for Wrangler CLI) - [Install Node.js](https://nodejs.org/)
- **Cloudflare Account** (free tier available) - [Sign up](https://dash.cloudflare.com/sign-up)
- **wasm-pack** (for testing) - Installed automatically via scripts

### 🚀 Quick Start

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd rust-api
   ```

2. **Set up environment configuration**
   ```bash
   cp .env.example .env
   # Edit .env with your deployed worker URL
   ```

3. **Install Wrangler CLI**
   ```bash
   npm install -g wrangler
   ```

4. **Login to Cloudflare**
   ```bash
   wrangler login
   ```

5. **Build and test locally**
   ```bash
   make build    # Build the Worker
   make dev      # Start local development server (http://localhost:8787)
   ```

6. **Run tests**
   ```bash
   make test     # Run all tests (unit + integration)
   ```

7. **Deploy to Cloudflare Workers**
   ```bash
   make deploy
   ```

### 📋 Available Commands

Use the provided Makefile for development tasks:

```bash
# 🏗️ Building & Development
make build          # Build the Cloudflare Worker
make dev            # Start local development server (http://localhost:8787)
make clean          # Clean build artifacts and coverage reports

# 🧪 Testing
make test           # Run all tests (unit + integration)
make test-unit      # Run only WASM unit tests
make test-integration # Run only integration tests (auto-detects local/deployed)
make test-local     # Run integration tests on local dev server

# 📊 Code Coverage
make coverage       # Generate code coverage reports (HTML, XML, JSON)
make coverage-ci    # Generate XML coverage report for CI/CD

# 🚀 Deployment
make deploy         # Deploy to Cloudflare Workers

# 🔧 Code Quality
make format         # Format code with rustfmt
make lint           # Run clippy linter
make check          # Run format, lint, and build

# 📖 Documentation
make docs           # Generate Redoc API documentation (api-docs.html)

# ❓ Help
make help           # Show all available commands
```

### 💻 Local Development

Run the worker locally with hot-reload support:

```bash
make dev
# or directly:
npx wrangler dev
```

This starts a local development server at `http://localhost:8787` that simulates the Cloudflare Workers environment with:
- 🔄 **Hot Reload**: Automatically rebuilds on code changes
- ⚡ **Fast Compilation**: Optimized WASM builds
- 🌐 **Workers Runtime**: Identical to production environment
- 🔍 **Debug Console**: Interactive debugging tools

**Testing the local server:**
```bash
# Test suggest endpoint
curl "http://localhost:8787/suggest?term=gen%201:1"

# Auto-test with our script
make test-local
```

### 🧪 Testing

The project supports comprehensive testing with multiple approaches:

#### Unit Tests (WebAssembly)
```bash
make test-unit
# or directly:
wasm-pack test --node
```
**Features:**
- 50+ comprehensive unit tests covering core functions
- Bible reference parser testing with multiple formats
- Context-aware reference parsing validation
- Edge case validation (empty queries, invalid books, complex patterns)

#### Integration Tests
```bash
# 🔍 Auto-detect local vs deployed
make test-integration

# 🏠 Force local testing
make test-local

# 🌐 Force deployed testing
WORKER_DEPLOYED_URL=https://your-worker.workers.dev make test-integration

# 🧪 Run all tests
make test
```

**Integration test features:**
- Validates live API endpoints
- Tests query parameter handling
- Verifies error responses
- URL encoding support testing

#### Environment Variables
Configure testing URLs with environment variables:

```bash
# Run all unit tests
make test

# Run tests with verbose output (shows println! output)
make test-verbose

# Run tests directly with cargo
cargo test --lib
```

**Test features:**
- ✅ **50+ unit tests**: Comprehensive coverage of parsers, validators, and response builders
- 🧪 **Rust test runner**: Professional test framework with proper assertions
- 📊 **Fast execution**: All tests run in ~0.05s
- 🔍 **Edge case coverage**: Tests invalid inputs, empty strings, complex patterns

### 📊 Code Coverage

Generate comprehensive code coverage reports with professional-grade tooling:

```bash
# 📈 Generate all coverage reports
make coverage

# 🤖 Generate XML only for CI/CD
make coverage-ci

# 🧹 Clean coverage artifacts
make clean  # Also removes coverage/ directory
```

#### Coverage Reports
The coverage system generates multiple report formats:
- **📄 HTML Report**: `./coverage/tarpaulin-report.html` (opens automatically in browser)
- **🔄 XML Report**: `./coverage/cobertura.xml` (for CI/CD integration)
- **📊 JSON Report**: `./coverage/tarpaulin-report.json` (for programmatic analysis)

#### Coverage Features
- 📊 **Line Coverage**: Shows which lines of code are executed during tests
- 🎯 **Function Coverage**: Tracks function execution and validates all code paths
- 🔍 **Branch Coverage**: Analyzes conditional branches and decision points
- 📈 **Multiple Formats**: HTML (visual inspection), XML (CI/CD), JSON (automation)
- 🧹 **Clean Integration**: Automatically cleaned with `make clean`
- 🔧 **CI/CD Ready**: Separate XML-only target for automated pipelines

#### Current Coverage Stats
- **Coverage**: Comprehensive test coverage of core functions
- **Tests**: 50+ unit tests passing
- **Functions**: Bible parsers, validators, and response builders fully tested

## 📁 Project Structure

```
rust-api/
├── 📄 Cargo.toml                 # Project dependencies and WebAssembly config
├── ⚙️ wrangler.toml              # Cloudflare Workers deployment configuration
├── 🛠️ Makefile                   # Development automation and task runner
├── 📝 README.md                  # This comprehensive documentation
├── 📖 API_REFERENCE.md           # Quick API reference with examples
├── 📚 SWAGGER.md                 # Swagger/OpenAPI documentation guide
├── 🔧 openapi.yaml               # OpenAPI 3.0 specification
├── 📮 postman_collection.json    # Postman collection for API testing
├── 🗄️ DATABASE_SCHEMA.md         # Database schema documentation
├── 🌍 .env.example               # Environment variables template
├── 🔒 .env                       # Local environment configuration (git-ignored)
├── 📂 scripts/                   # Build and utility scripts
│   ├── 🏗️ build-worker.sh        # Worker build automation
│   └── 📊 coverage.sh            # Code coverage script
├── 📂 src/                       # Rust source code
│   ├── 📋 lib.rs                 # Main worker logic and API handlers
│   ├── 🧪 lib_spec.rs            # Unit tests
│   ├── 📦 types.rs               # Type definitions
│   ├── 🔨 response_builders.rs   # Response helper functions
│   └── 📂 parser/                # Bible reference parsing logic
│       ├── 📖 books.rs           # Book definitions and validation
│       ├── 🔍 formatting.rs      # Reference formatting utilities
│       └── 🧠 mod.rs             # Smart parsing logic
└── 📂 build/                     # Generated WebAssembly files (after build)
    ├── 📂 worker/
    │   ├── 🔗 shim.mjs           # JavaScript shim for the worker
    │   └── ⚡ rust_api_bg.wasm    # Compiled WebAssembly binary
    └── 📊 coverage/              # Generated coverage reports (HTML, XML, JSON)
```

## ⚙️ Configuration

### wrangler.toml

The `wrangler.toml` file contains your worker configuration:

```toml
name = "rust-api"
main = "build/worker/shim.mjs"
compatibility_date = "2025-01-17"

[build]
command = "cargo install -q worker-build && worker-build --release"

[env.production]
vars = { ENVIRONMENT = "production" }

[env.staging]
vars = { ENVIRONMENT = "staging" }
```

### Environment Variables

#### .env Configuration
Create a `.env` file for local development:

```bash
# Copy the example file
cp .env.example .env

# Edit with your values
WORKER_DEPLOYED_URL=https://rust-api.your-subdomain.workers.dev
```

#### Wrangler Environment Variables
Set environment variables in `wrangler.toml` for different deployment environments:

```toml
[env.production]
  ENVIRONMENT = "production"
  LOG_LEVEL = "info"

[env.staging]
  ENVIRONMENT = "staging" 
  LOG_LEVEL = "debug"
```

#### Runtime Environment Variables
Access environment variables in your Rust code:

```rust
#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    let environment = env.var("ENVIRONMENT")?.to_string();
    // Use environment variable...
}
```

## 📦 Dependencies

Core production dependencies:
- **worker**: Cloudflare Workers runtime for Rust
- **serde**: JSON serialization framework for Rust
- **console_error_panic_hook**: Enhanced panic messages in WebAssembly

Development and testing dependencies:
- **js-sys**: JavaScript bindings for WebAssembly
- **wasm-bindgen**: WebAssembly ↔ JavaScript interop layer
- **wasm-bindgen-test**: WebAssembly testing framework
- **cargo-tarpaulin**: Code coverage analysis tool

## 🚀 Deployment

### Using Wrangler CLI

1. **Build the worker**:
   ```bash
   make build
   ```

2. **Deploy to production**:
   ```bash
   make deploy
   # or directly:
   wrangler deploy --env=""
   ```

3. **Deploy to staging**:
   ```bash
   wrangler deploy --env staging
   ```

### CI/CD Integration

#### GitHub Actions Example
For automated deployments with comprehensive testing:

```yaml
name: Deploy Rust API Worker
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown
      
      - name: Install tarpaulin
        run: cargo install cargo-tarpaulin
      
      - name: Run tests and coverage
        run: |
          make test
          make coverage-ci
      
      - name: Upload coverage reports
        uses: codecov/codecov-action@v3
        with:
          file: ./coverage/cobertura.xml

  deploy:
    needs: test
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown
      
      - name: Build Worker
        run: make build
      
      - name: Deploy to Cloudflare Workers
        uses: cloudflare/wrangler-action@v3
        with:
          apiToken: ${{ secrets.CLOUDFLARE_API_TOKEN }}
          command: deploy --env=""
```

## ⚡ Performance

Real-world performance metrics from production deployment:

- **🚀 Cold start**: ~1ms (WebAssembly initialization)
- **⚡ Execution**: Sub-millisecond response times
- **💾 Memory usage**: ~1-2MB per request
- **📦 Bundle size**: ~350KB uncompressed, ~144KB gzipped
- **🌍 Global latency**: <50ms worldwide (Cloudflare's edge network)
- **📈 Throughput**: 1000+ requests/second per edge location

### Performance Optimizations
- **WebAssembly compilation**: Optimized with `--release` builds
- **Smart caching**: Static assets cached at edge
- **Minimal dependencies**: Lean runtime for fast cold starts
- **Edge computing**: Code runs close to users globally

## 📊 Monitoring

Cloudflare provides comprehensive built-in analytics for your worker:

### 📈 Available Metrics
- **Request Analytics**: Count, success/error rates, geographic distribution
- **Performance Metrics**: CPU usage, memory consumption, response time percentiles
- **Error Tracking**: Detailed error logs with stack traces
- **Geographic Insights**: Request patterns by country/region
- **Edge Caching**: Cache hit rates and performance improvements

### 🔍 Monitoring Tools

**Cloudflare Dashboard:**
Access analytics at: Cloudflare Dashboard → Workers → rust-api → Analytics

**Real-time Logs:**
```bash
# Stream live worker logs
wrangler tail

# Filter by status code
wrangler tail --status error

# Filter by specific messages
wrangler tail --search "error"
```

**Custom Logging:**
```rust
// Add structured logging in your worker
console_log!("Request processed: {} ms", duration);
worker::console_error!("Error processing request: {}", error);
```

## 🔧 Troubleshooting

### Build Issues

**Common build errors and solutions:**

1. **Missing Rust WebAssembly target:**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

2. **worker-build not found:**
   ```bash
   cargo install worker-build
   ```

3. **Out-of-date dependencies:**
   ```bash
   make clean
   cargo update
   make build
   ```

4. **Permission issues with scripts:**
   ```bash
   chmod +x scripts/*.sh
   ```

### Runtime Issues

**Debug worker runtime problems:**

1. **Check worker logs:**
   ```bash
   wrangler tail
   ```

2. **Test locally first:**
   ```bash
   make dev
   make test-local
   ```

3. **Validate build artifacts:**
   ```bash
   ls -la build/worker/
   # Should contain: shim.mjs and rust_api_bg.wasm
   ```

### Testing Issues

**Common testing problems:**

1. **Integration tests failing:**
   ```bash
   # Check if worker is deployed
   curl "https://api.nyiregyhazimetodista.hu/suggest?term=gen%201:1"
   
   # Update .env with correct URL
   echo "WORKER_DEPLOYED_URL=https://your-actual-url.workers.dev" > .env
   ```

2. **Local tests failing:**
   ```bash
   # Ensure local server is running
   make dev
   # In another terminal:
   make test-local
   ```

3. **Coverage generation failing:**
   ```bash
   # Install tarpaulin
   cargo install cargo-tarpaulin
   make coverage
   ```

## 🤝 Contributing

We welcome contributions! Please follow these guidelines:

1. **Fork the repository** and create a feature branch
2. **Run tests**: `make test` and ensure all tests pass
3. **Check code quality**: `make check` (format + lint + build)
4. **Add tests** for new functionality
5. **Update documentation** as needed
6. **Submit a pull request** with a clear description

### Development Workflow
```bash
# Set up development environment
make dev

# Make your changes...

# Test your changes
make test

# Check code quality
make check

# Generate coverage report
make coverage

# Clean up
make clean
```

## 📄 License

This project is open source and available under the [MIT License](LICENSE).

---

## 📚 Additional Resources

- **[Cloudflare Workers Documentation](https://developers.cloudflare.com/workers/)**
- **[Rust WebAssembly Book](https://rustwasm.github.io/docs/book/)**
- **[worker-rs Documentation](https://docs.rs/worker/latest/worker/)**
- **[Wrangler CLI Reference](https://developers.cloudflare.com/workers/wrangler/)**
- **[WebAssembly.org](https://webassembly.org/)**

## 🚀 Live Demo

**Production API**: https://api.nyiregyhazimetodista.hu/suggest?term=gen%201:1

Try it yourself:
```bash
# Simple verse lookup
curl "https://api.nyiregyhazimetodista.hu/suggest?term=gen%201:1"

# Multiple references
curl "https://api.nyiregyhazimetodista.hu/suggest?term=1Kor%202,2-3;Jn%201,1-2"

# Contextual parsing
curl "https://api.nyiregyhazimetodista.hu/suggest?term=mt5,1-2;5-6"
```

---

*Built with ❤️ using Rust, WebAssembly, and Cloudflare Workers*


Run unit tests
// cargo test --lib -- --nocapture

Update linters
// cargo clippy --fix --allow-dirty --allow-staged