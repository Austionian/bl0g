FROM rust:1.81 AS builder

WORKDIR /app
COPY . . 
RUN cargo build --release --bin bl0g

FROM gcr.io/distroless/cc-debian12 AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/bl0g bl0g
COPY config config
COPY assets assets
COPY templates templates
COPY content content
ENV APP_ENVIRONMENT=production 
ENTRYPOINT ["./bl0g"]
