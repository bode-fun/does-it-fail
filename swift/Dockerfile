FROM --platform=linux/amd64 swift:5.7.3-jammy AS builder

WORKDIR /usr/src/ReadFile
COPY . .
RUN swift build -c release

FROM --platform=linux/amd64 swift:5.7.3-jammy-slim

WORKDIR /app
COPY ./README.md ./README.md
COPY --from=builder /usr/src/ReadFile/.build/release/ReadFile /usr/local/bin/ReadFile

ENTRYPOINT [ "ReadFile" ]