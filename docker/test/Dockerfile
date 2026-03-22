FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive
ENV RUST_BACKTRACE=1

RUN apt-get update && apt-get install -y \
    curl git build-essential pkg-config \
    libssl-dev libdbus-1-dev libsecret-1-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Rust stable toolchain
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --default-toolchain stable
ENV PATH="/root/.cargo/bin:${PATH}"

# Minimal Claude Code stub so doctor can find the directory
RUN mkdir -p /root/.claude

WORKDIR /workspace
COPY . .

RUN cargo build -p cst-cli

CMD ["cargo", "test", "-p", "cst-cli", "--test", "cli_integration", "--", "--nocapture"]
