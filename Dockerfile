# Use Rust official image as base
FROM rust:1.75-slim as builder

# Set working directory
WORKDIR /app

# Copy the entire project
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false app

# Set working directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/helixor /app/helixor

# Copy any config files if needed
COPY --from=builder /app/*.toml /app/

# Change ownership to app user
RUN chown -R app:app /app

# Switch to app user
USER app

# Expose ports for HTTP and gRPC
EXPOSE 8080 9090

# Set environment variables
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Run the application
CMD ["/app/helixor", "serve"] 