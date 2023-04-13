FROM rust:1.68 as builder
WORKDIR /usr/src/service

RUN apt-get update && apt-get install -y protobuf-compiler

COPY Cargo.* ./
COPY ./src ./src
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y curl openssl libssl-dev libpq-dev
RUN /usr/sbin/update-ca-certificates
COPY --from=builder /usr/local/cargo/bin/subscribe /usr/local/bin/subscribe

CMD ["subscribe"]
