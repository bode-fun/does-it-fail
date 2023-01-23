FROM rust:1.66.1-alpine3.17 AS builder

WORKDIR /usr/src/read_file
COPY . .
RUN cargo build --release

FROM ubuntu:22.04
COPY --from=builder /usr/src/read_file/target/release/read_file /usr/local/bin/read_file

WORKDIR /app
COPY ./README.md ./README.md

ENTRYPOINT [ "read_file" ]