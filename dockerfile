FROM rust:1-slim AS builder
WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src

COPY src ./src
RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim
RUN useradd --create-home appuser
COPY --from=builder /usr/src/app/target/release/docker-practica /usr/local/bin/docker-practica

USER appuser
EXPOSE 5000
CMD ["docker-practica"]