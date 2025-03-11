FROM rust:1.74.1 as build-env
WORKDIR /app
COPY . /app
RUN cargo build --release

# FROM gcr.io/distroless/cc
FROM cgr.dev/chainguard/glibc-dynamic
COPY --from=build-env /app/target/release/re-server /
COPY .env /
COPY .key.json /
CMD ["./server"]