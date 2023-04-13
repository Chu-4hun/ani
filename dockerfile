FROM lukemathwalker/cargo-chef:latest-rust-1-alpine3.17 AS builder
WORKDIR /app

COPY Cargo.toml .
COPY Cargo.lock .
COPY src/main.rs src/main.rs
RUN cargo chef prepare --recipe-path recipe.json

RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY . .
ARG SQLX_OFFLINE=true
RUN cargo build --release --target x86_64-unknown-linux-musl --bin rust_ani


FROM alpine AS runtime
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rust_ani /usr/local/bin/rust_ani
CMD ["/usr/local/bin/rust_ani"]
