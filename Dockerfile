FROM rust:1.76-slim AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y \
    pkg-config cmake \
    libx11-dev libxcb1-dev libxkbcommon-dev \
    libwayland-dev libvulkan-dev \
    && rm -rf /var/lib/apt/lists/*

# Cache deps
COPY Cargo.toml Cargo.lock ./
COPY newter-compiler/Cargo.toml ./newter-compiler/
COPY newter-lsp/Cargo.toml ./newter-lsp/
COPY newter-terminal/Cargo.toml ./newter-terminal/
RUN mkdir -p newter-compiler/src newter-lsp/src newter-terminal/src && \
    echo "fn main(){}" > newter-compiler/src/main.rs && \
    echo "fn main(){}" > newter-lsp/src/main.rs && \
    echo "fn main(){}" > newter-terminal/src/main.rs && \
    cargo build -p newter-compiler --release 2>/dev/null || true && \
    rm -rf newter-compiler/src newter-lsp/src newter-terminal/src

# Build actual source
COPY . .
RUN touch newter-compiler/src/main.rs && \
    cargo build -p newter-compiler --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/newter-compiler /usr/local/bin/newter-compiler
COPY examples/ ./examples/
EXPOSE 3333
CMD ["newter-compiler", "serve", "examples/dashboard.newt", "--port", "3333"]
