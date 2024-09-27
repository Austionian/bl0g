FROM rust:1.81 AS chef

WORKDIR /app

RUN cargo install --locked cargo-chef sccache
ENV RUSTC_WRAPPER=sccache SCCACHE_DIR=/sccache

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json
COPY . . 
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo build --release

FROM gcr.io/distroless/cc-debian12 AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/bl0g bl0g
COPY config config
COPY assets assets
COPY templates templates
COPY content content
ENV APP_ENVIRONMENT=production 
ENTRYPOINT ["./bl0g"]
