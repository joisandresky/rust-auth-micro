FROM rust:1.78.0-alpine3.19 as builder
RUN apk add --no-cache musl-dev openssl-dev pkgconfig protobuf-dev protoc

WORKDIR /usr/src/rust-auth-micro
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release

FROM alpine:3.19.0
RUN apk add --no-cache openssl ca-certificates
COPY --from=builder /usr/src/rust-auth-micro/target/x86_64-unknown-linux-musl/release/rust-auth-micro /
CMD [ "./rust-auth-micro" ]