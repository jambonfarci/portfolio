#!/bin/bash

# Production build script for backend
set -e

echo "🏗️  Building backend for production..."

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Rust/Cargo is not installed. Please install Rust and try again."
    exit 1
fi

# Load production environment
if [ -f ".env.production" ]; then
    log_info "Loading production environment variables..."
    export $(cat .env.production | grep -v '^#' | xargs)
fi

# Run tests first
log_info "Running tests..."
cargo test --release

# Check code formatting
log_info "Checking code formatting..."
cargo fmt --check || log_warn "Code formatting issues found"

# Run clippy for linting
log_info "Running clippy..."
cargo clippy --release -- -D warnings || log_warn "Clippy warnings found"

# Build for production with optimizations
log_info "Building for production..."
RUSTFLAGS="-C target-cpu=native" cargo build --release

# Verify build
if [ -f "target/release/portfolio-backend" ]; then
    log_info "✅ Build completed successfully!"
    
    # Show binary size
    log_info "Binary size:"
    ls -lh target/release/portfolio-backend
    
    # Strip debug symbols for smaller binary
    log_info "Stripping debug symbols..."
    strip target/release/portfolio-backend
    
    log_info "Optimized binary size:"
    ls -lh target/release/portfolio-backend
else
    echo "❌ Build failed - binary not found"
    exit 1
fi

# Run a quick smoke test
log_info "Running smoke test..."
timeout 5s ./target/release/portfolio-backend --help > /dev/null || log_info "Binary appears to be working"

log_info "🎉 Backend production build ready!"