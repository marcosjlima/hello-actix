# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------
FROM rust:alpine as builder

RUN apk update 
RUN apk add --no-cache musl-dev
RUN apk add --no-cache libpq

WORKDIR /usr/src/hello-actix

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY src ./src

RUN rustup target add x86_64-unknown-linux-musl
RUN rustup update && cargo update

RUN cargo build --release --target=x86_64-unknown-linux-musl
RUN cargo install --target x86_64-unknown-linux-musl --path .

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/hello-actix*

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------
FROM alpine:latest

EXPOSE 8082

ENV TZ=Etc/UTC 

RUN apk update 
RUN apk add --no-cache ca-certificates tzdata
RUN rm -rf /var/cache/apk/*
    
COPY --from=builder /usr/local/cargo/bin/hello-actix /usr/local/bin/hello-actix

CMD ["hello-actix"]