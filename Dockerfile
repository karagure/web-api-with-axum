# ---- Build stage ----
# edition = "2024" requires Rust >= 1.85
FROM rust:1.90-slim-bookworm AS builder

WORKDIR /app

# Cache dependencies: copy manifests first and build a dummy main.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src

# Build the real application.
COPY src ./src
RUN touch src/main.rs && cargo build --release

# ---- Runtime stage ----
FROM debian:bookworm-slim AS runtime

# Run as a non-root user.
RUN useradd --create-home --user-group app
USER app
WORKDIR /home/app

COPY --from=builder /app/target/release/tp-wik-dps-01 /usr/local/bin/tp-wik-dps-01

# Configurable via env (see .env). Defaults: PORT=3000, INSTANCE_ID=hostname.
ENV PORT=3000
EXPOSE 3000

CMD ["tp-wik-dps-01"]
