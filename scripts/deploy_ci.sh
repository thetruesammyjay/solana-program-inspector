#!/bin/bash
# CI/CD Deployment Script for Solana Program Inspector
# Usage: ./scripts/deploy_ci.sh [--test|--prod]

set -euo pipefail

# Configuration
REPO="thetruesammyjay/solana-program-inspector"
TAG=$(git describe --tags --always)
BUILD_DIR="target/release"
ARTIFACTS=(
  "solana-program-inspector"
  "ghidra_plugin.py"
  "signatures.db"
)

# Parse arguments
MODE="test"
if [[ $# -gt 0 ]]; then
  case $1 in
    --prod) MODE="prod" ;;
    --test) MODE="test" ;;
    *) echo "Usage: $0 [--test|--prod]"; exit 1 ;;
  esac
fi

# Build release
echo "🔧 Building release..."
cargo build --release

# Verify artifacts
echo "🔍 Verifying artifacts..."
for artifact in "${ARTIFACTS[@]}"; do
  if [[ ! -f "$BUILD_DIR/$artifact" ]]; then
    echo "❌ Missing artifact: $artifact"
    exit 1
  fi
done

# Run tests
echo "🧪 Running tests..."
cargo test --release

# Deployment logic
if [[ "$MODE" == "prod" ]]; then
  echo "🚀 Deploying to production..."
  
  # Create release package
  mkdir -p "release-$TAG"
  cp -r $BUILD_DIR/*.{so,dll,exe} "release-$TAG/"
  cp scripts/ghidra_plugin.py "release-$TAG/"
  cp src/extractor/signatures.db "release-$TAG/"
  
  # Generate checksums
  pushd "release-$TAG"
  sha256sum * > SHA256SUMS
  popd
  
  echo "✅ Release package created: release-$TAG"
  
  # Here you would add actual deployment commands like:
  # gh release create "$TAG" "release-$TAG/*" --title "v$TAG"
else
  echo "🧪 Test deployment complete (dry run)"
fi

echo "🎉 Deployment script finished"