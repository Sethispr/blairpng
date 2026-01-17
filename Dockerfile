FROM rust:1-slim-bookworm as builder

WORKDIR /usr/src/blairpng

# copy deps files
COPY Cargo.toml Cargo.lock ./

# build deps but fake main
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# read
COPY src ./src
RUN touch src/main.rs
RUN cargo build --release

# runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/blairpng/target/release/blairpng /usr/local/bin/blairpng
WORKDIR /data
ENTRYPOINT ["blairpng"]
CMD ["--help"]
