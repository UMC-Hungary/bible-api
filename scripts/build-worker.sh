#!/bin/bash

echo "🔧 Installing worker-build if not already installed..."
cargo install -q worker-build

echo "🏗️  Building Cloudflare Worker..."
worker-build --release

echo "✅ Build complete! Check the build/ directory for generated files."
echo ""
echo "📚 Next steps:"
echo "  1. Install Wrangler CLI: npm install -g @cloudflare/wrangler"
echo "  2. Login to Cloudflare: npx wrangler login"
echo "  3. Deploy: npx wrangler deploy"
echo "  4. Test locally: npx wrangler dev"
