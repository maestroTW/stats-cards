FROM rust:1.84.1-alpine3.21 as builder

WORKDIR /usr/src/app

RUN apk add libc-dev openssl-dev openssl-libs-static

COPY Cargo.toml Cargo.lock ./
COPY templates templates
COPY src src
COPY data/lang2hex.json data/lang2hex.json
RUN cargo build --release

FROM alpine:latest

COPY --from=builder /usr/src/app/target/release/stats-cards /usr/local/bin/stats-cards
ENV SERVICE_HOST=0.0.0.0

CMD ["stats-cards"]