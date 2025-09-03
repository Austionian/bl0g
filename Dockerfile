FROM --platform=$BUILDPLATFORM rust:1-bullseye AS chef

WORKDIR /app

USER root:root

RUN apt update && apt upgrade -y
RUN apt install -y gcc-aarch64-linux-gnu

RUN rustup target add aarch64-unknown-linux-musl

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
    cargo build --release --target aarch64-unknown-linux-musl

FROM --platform=$BUILDPLATFORM gcr.io/distroless/cc-debian12 AS runtime
WORKDIR /app
COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/bl0g bl0g
COPY config config
COPY assets assets
COPY templates templates
COPY content content
ENV APP_ENVIRONMENT=production 
ENTRYPOINT ["./bl0g"]
