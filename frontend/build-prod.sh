#!/bin/bash

# Production build script for frontend
set -e

echo "🏗️  Building frontend for production..."

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

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "Node.js is not installed. Please install Node.js and try again."
    exit 1
fi

# Check if npm is installed
if ! command -v npm &> /dev/null; then
    echo "npm is not installed. Please install npm and try again."
    exit 1
fi

# Load production environment
if [ -f ".env.production" ]; then
    log_info "Loading production environment variables..."
    export $(cat .env.production | grep -v '^#' | xargs)
fi

# Install dependencies
log_info "Installing dependencies..."
npm ci --only=production

# Run type checking
log_info "Running type checks..."
npm run check

# Run linting
log_info "Running linter..."
npm run lint

# Run tests
log_info "Running tests..."
npm run test

# Build for production
log_info "Building application..."
npm run build

# Verify build
if [ -d "build" ]; then
    log_info "✅ Build completed successfully!"
    log_info "Build size:"
    du -sh build/
    
    # List main files
    log_info "Main build files:"
    find build -name "*.js" -o -name "*.css" -o -name "*.html" | head -10
else
    echo "❌ Build failed - build directory not found"
    exit 1
fi

log_info "🎉 Frontend production build ready!"