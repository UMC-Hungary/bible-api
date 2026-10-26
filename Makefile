.PHONY: help build test test-verbose test-watch coverage coverage-ci clean dev deploy format lint check docs

# Default target
help:
	@echo "Available targets:"
	@echo "  build          - Build the Cloudflare Worker"
	@echo "  test           - Run all Rust unit tests (cargo test)"
	@echo "  test-verbose   - Run tests with verbose output"
	@echo "  test-watch     - Run tests in watch mode (requires cargo-watch)"
	@echo "  coverage       - Generate code coverage reports (HTML, XML, JSON)"
	@echo "  coverage-ci    - Generate XML coverage report for CI/CD"
	@echo "  clean          - Clean build artifacts and coverage reports"
	@echo "  dev            - Run in development mode with Wrangler (http://localhost:8787)"
	@echo "  deploy         - Deploy to Cloudflare Workers"
	@echo "  format         - Format code with rustfmt"
	@echo "  lint           - Run clippy linter"
	@echo "  check          - Run format, lint, and build"
	@echo "  docs           - Generate Swagger UI documentation"

build:
	@./scripts/build-worker.sh

test:
	@echo "🧪 Running Rust unit tests..."
	@cargo test --lib

test-verbose:
	@echo "🧪 Running Rust unit tests (verbose)..."
	@cargo test --lib -- --nocapture

test-watch:
	@echo "🧪 Running tests in watch mode..."
	@cargo watch -x "test --lib"

coverage:
	@echo "📊 Generating code coverage reports..."
	@mkdir -p coverage
	@cargo tarpaulin --out html --out xml --out json --output-dir ./coverage --exclude-files target/* --exclude-files build/* --exclude-files pkg/* --timeout 120 --skip-clean --target-dir ./target/coverage
	@echo ""
	@echo "✅ Coverage reports generated!"
	@echo "   HTML: ./coverage/tarpaulin-report.html"
	@echo "   XML:  ./coverage/cobertura.xml"
	@echo "   JSON: ./coverage/tarpaulin-report.json"
	@if command -v open >/dev/null 2>&1; then \
		echo "🌐 Opening HTML report in browser..."; \
		open ./coverage/tarpaulin-report.html; \
	fi

coverage-ci:
	@echo "🤖 Generating XML coverage report for CI..."
	@mkdir -p coverage
	@cargo tarpaulin --out xml --output-dir ./coverage --exclude-files target/* --exclude-files build/* --exclude-files pkg/* --timeout 120 --skip-clean --target-dir ./target/coverage

clean:
	cargo clean
	rm -rf build/
	rm -rf pkg/
	rm -rf coverage/
	rm -f api-docs.html

dev:
	npx wrangler dev

deploy:
	npx wrangler deploy --env="production"

format:
	cargo fmt

lint:
	cargo clippy --

check: format lint build

docs:
	@echo "📖 Generating API documentation with Redoc..."
	@if ! command -v redocly >/dev/null 2>&1; then \
		echo "⚠️  redocly CLI not found. Installing..."; \
		npm install -g @redocly/cli; \
	fi
	@redocly build-docs openapi.yaml -o api-docs.html
	@echo "✅ Generated: api-docs.html"
