FROM rust:1.66.1-alpine3.17

WORKDIR /usr/src/read_file
COPY . .
RUN cargo install --path .

WORKDIR /app
COPY ./README.md ./README.md
RUN rm -rf /usr/src/read_file

CMD ["read_file"]