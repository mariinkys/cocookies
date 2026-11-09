# ---- Build -------------------------------------------------------------------------------
FROM docker.io/library/rust:1-alpine AS build
RUN apk add --no-cache musl-dev
WORKDIR /app

# Build dependencies first so they are cached until Cargo.toml / Cargo.lock change.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src \
    && echo 'fn main() {}' > src/main.rs \
    && touch src/lib.rs \
    && cargo build --release --locked \
    && rm -rf src

COPY . .
RUN touch src/main.rs src/lib.rs && cargo build --release --locked

# ---- Runtime -----------------------------------------------------------------------------
FROM docker.io/library/alpine:3
RUN apk add --no-cache ca-certificates \
    && addgroup -S app && adduser -S app -G app \
    && mkdir -p /app/data && chown app:app /app/data

COPY --from=build /app/target/release/cocookies /usr/local/bin/cocookies

USER app
WORKDIR /app
ENV BIND_ADDR=0.0.0.0:8080 \
    DB_URL=/app/data/cocookies.db \
    RUST_LOG=info,sqlx=warn

EXPOSE 8080
HEALTHCHECK --interval=30s --timeout=3s --start-period=10s \
    CMD wget -q -O /dev/null http://127.0.0.1:8080/api/system/health || exit 1
ENTRYPOINT ["/usr/local/bin/cocookies"]
