FROM lukemathwalker/cargo-chef as planner
WORKDIR app
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM lukemathwalker/cargo-chef as cacher
WORKDIR app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust as builder
WORKDIR app
COPY . .
COPY .env.container ./.env
# Copy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME

ARG SQLX_OFFLINE=true
RUN cargo build --release --bin rust_ani

FROM rust as runtime
WORKDIR app
COPY --from=builder /app/target/release/rust_ani /usr/local/bin
ENTRYPOINT ["/usr/local/bin/rust_ani"]