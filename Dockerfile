# Multi-stage build for Self-Evolving Memory

# Stage 1: Build Rust backend
FROM rust:1.75-bookworm as builder

WORKDIR /app

# Copy Cargo files first for dependency caching
COPY Cargo.toml Cargo.lock* ./
COPY src ./src

# Build the application
RUN cargo build --release

# Stage 2: Build Web UI
FROM node:20-alpine as web-builder

WORKDIR /app

COPY web-ui/package.json web-ui/package-lock.json* ./web-ui/
WORKDIR /app/web-ui
RUN npm ci

COPY web-ui ./
RUN npm run build

# Stage 3: Final runtime image
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy built binaries
COPY --from=builder /app/target/release/self_evolving_memory /app/server

# Copy built web UI
COPY --from=web-builder /app/web-ui/dist /app/web-ui/dist

# Create non-root user
RUN useradd -m -u 1000 appuser && \
    chown -R appuser:appuser /app

USER appuser

# Expose ports
EXPOSE 3000 4000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# Set environment variables
ENV RUST_LOG=info
ENV MEMORY_API_PORT=3000
ENV MEMORY_MCP_PORT=4000

# Run the server
CMD ["./server"]