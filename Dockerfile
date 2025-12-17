# Stage 1: Build frontend
FROM oven/bun:1-alpine AS frontend-builder
WORKDIR /app
COPY web/package.json web/bun.lock* ./
RUN bun install --frozen-lockfile
COPY web/ ./
RUN bun run build

# Stage 2: Build backend dependencies (cached layer)
FROM rust:1.85-alpine AS backend-deps
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src target/release/deps/splitdumb*

# Stage 3: Build backend binary
FROM backend-deps AS backend-builder
COPY src/ ./src/
RUN cargo build --release

# Stage 4: Runtime
FROM alpine:3.21
RUN apk add --no-cache ca-certificates nginx

COPY --from=backend-builder /app/target/release/splitdumb /usr/local/bin/
COPY --from=frontend-builder /app/dist /var/www/html

RUN rm -f /etc/nginx/http.d/default.conf
COPY nginx.conf /etc/nginx/http.d/default.conf

RUN mkdir -p /data /run/nginx
WORKDIR /data

COPY docker-entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/docker-entrypoint.sh

EXPOSE 80

CMD ["docker-entrypoint.sh"]
