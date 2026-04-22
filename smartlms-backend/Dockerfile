FROM rust:1.75-slim AS builder

WORKDIR /build

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy and build backend
COPY smartlms-backend .
RUN cargo build --release --bin smartlms-api

# Final stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 appuser

# Copy binary
COPY --from=builder /build/target/release/smartlms-api /usr/local/bin/smartlms-api

# Copy config
COPY smartlms-backend/config /app/config

WORKDIR /app
RUN chown -R appuser:appuser /app

USER appuser

EXPOSE 8080

ENV RUST_LOG=info \
    RUST_BACKTRACE=1

CMD ["smartlms-api", "--config", "/app/config"]