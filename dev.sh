#!/bin/bash

# Development startup script for Portfolio project

echo "🚀 Starting Portfolio Development Environment"
echo "============================================="

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker is not running. Please start Docker first."
    exit 1
fi

# Create data directory if it doesn't exist
mkdir -p data

# Copy environment files if they don't exist
if [ ! -f backend/.env ]; then
    cp backend/.env.example backend/.env
    echo "✅ Created backend/.env from example"
fi

if [ ! -f frontend/.env ]; then
    cp frontend/.env.example frontend/.env
    echo "✅ Created frontend/.env from example"
fi

echo "🐳 Starting services with Docker Compose..."
docker compose up --build

echo "🎉 Development environment started!"
echo "📱 Frontend: http://localhost:5174"
echo "🔧 Backend API: http://localhost:3001"
echo ""
echo "Press Ctrl+C to stop all services"