FROM rust:1.54-slim AS chef
RUN cargo install cargo-chef --version 0.1.22

FROM chef AS recipe_builder
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS deps_builder
WORKDIR /app
COPY --from=recipe_builder /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.53-slim as builder
WORKDIR /app
COPY . .
COPY --from=deps_builder /app/target target
COPY --from=deps_builder $CARGO_HOME $CARGO_HOME
RUN cargo build --release

FROM rust:1.53-slim as runtime
WORKDIR /app
COPY --from=builder /app/target target
COPY . .
ENV PORT=80
EXPOSE 80
ENTRYPOINT ["cargo", "run", "--release"]