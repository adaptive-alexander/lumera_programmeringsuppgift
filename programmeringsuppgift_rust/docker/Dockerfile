# Operating System
FROM ubuntu:22.04

# Install perf
RUN apt-get update && apt-get install -y linux-tools-common linux-tools-generic linux-tools-`uname -r`

# Rust version
ENV RUST_VERSION=stable

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain=$RUST_VERSION

# Workdir
WORKDIR /app

# Copy
COPY . .

# Build Rust
RUN cargo install --path .

ENTRYPOINT ["app/programmeringsuppgift_rust"]
