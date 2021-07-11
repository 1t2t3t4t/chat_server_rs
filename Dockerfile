FROM rust:1.53-slim AS recipe_builder
RUN cargo install cargo-chef --version 0.1.22
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.53-slim AS deps_builder
RUN cargo install cargo-chef --version 0.1.22
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
COPY --from=builder /app/target/release/chat_server /usr/local/bin/chat_server
RUN ls /usr/local/bin
ENV PORT=80
EXPOSE 80
ENTRYPOINT ["chat_server"]