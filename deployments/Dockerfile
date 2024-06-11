FROM rust:1.76.0-alpine3.19 as builder
RUN apk add --no-cache musl-dev

WORKDIR /usr/src/rust-auth-micro
COPY . .
RUN cargo build --release

FROM alpine:3.19.0
COPY --from=builder /usr/src/rust-auth-micro/target/release/rust-auth-micro /
CMD [ "./rust-auth-micro" ]