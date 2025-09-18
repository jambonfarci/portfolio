#!/bin/bash

# Portfolio Deployment Script
set -e

echo "🚀 Starting portfolio deployment..."

# Configuration
ENVIRONMENT=${1:-production}
COMPOSE_FILE="docker-compose.prod.yml"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    log_error "Docker is not running. Please start Docker and try again."
    exit 1
fi

# Check if docker-compose is available
if ! command -v docker-compose &> /dev/null; then
    log_error "docker-compose is not installed. Please install it and try again."
    exit 1
fi

# Backup existing data (if any)
if [ -d "data" ]; then
    log_info "Backing up existing data..."
    cp -r data data_backup_$(date +%Y%m%d_%H%M%S)
fi

# Pull latest images (if using registry)
log_info "Pulling latest images..."
docker-compose -f $COMPOSE_FILE pull || log_warn "Could not pull images (building locally)"

# Build and start services
log_info "Building and starting services..."
docker-compose -f $COMPOSE_FILE up --build -d

# Wait for services to be healthy
log_info "Waiting for services to be healthy..."
sleep 10

# Check backend health
if curl -f http://localhost:3001/health > /dev/null 2>&1; then
    log_info "✅ Backend is healthy"
else
    log_error "❌ Backend health check failed"
    docker-compose -f $COMPOSE_FILE logs backend
    exit 1
fi

# Check frontend health
if curl -f http://localhost/health > /dev/null 2>&1; then
    log_info "✅ Frontend is healthy"
else
    log_error "❌ Frontend health check failed"
    docker-compose -f $COMPOSE_FILE logs frontend
    exit 1
fi

# Show running containers
log_info "Deployment completed successfully! 🎉"
log_info "Running containers:"
docker-compose -f $COMPOSE_FILE ps

log_info "Portfolio is now available at:"
log_info "  Frontend: http://localhost"
log_info "  Backend API: http://localhost:3001"

# Optional: Run smoke tests
if [ -f "scripts/smoke-tests.sh" ]; then
    log_info "Running smoke tests..."
    bash scripts/smoke-tests.sh
fi