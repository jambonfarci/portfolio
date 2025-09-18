# Multi-stage build for production
FROM rust:1.75 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY migrations ./migrations

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/portfolio-backend /app/portfolio-backend

# Copy migrations
COPY --from=builder /app/migrations /app/migrations

# Create data directory
RUN mkdir -p /app/data

# Expose port
EXPOSE 3001

# Set environment variables
ENV RUST_LOG=info
ENV DATABASE_URL=sqlite:///app/data/portfolio.db
ENV HOST=0.0.0.0
ENV PORT=3001

# Run the binary
CMD ["./portfolio-backend"]