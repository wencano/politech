# syntax=docker/dockerfile:1
# Target: Ubuntu 24.04 LTS (x86_64) — matches Dokploy host

FROM ubuntu:24.04 AS builder

ARG TRUNK_VERSION=0.21.14

# Node 20+ required by Tailwind CSS v4 (Ubuntu apt ships Node 18)
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl gnupg \
    && curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y --no-install-recommends \
        nodejs \
        build-essential \
        pkg-config \
        libssl-dev \
        git \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --default-toolchain none

ENV PATH="/root/.cargo/bin:${PATH}" \
    CARGO_TERM_COLOR=never \
    CARGO_PROFILE_RELEASE_LTO=false \
    CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16

RUN curl -fsSL \
        "https://github.com/trunk-rs/trunk/releases/download/v${TRUNK_VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz" \
    | tar xz -C /usr/local/bin

WORKDIR /app

COPY package.json ./
COPY web ./web
RUN npm install && npm run css:build

COPY rust-toolchain.toml Cargo.toml Cargo.lock ./
RUN rustup toolchain install

COPY src ./src
COPY migrations ./migrations
COPY index.html Trunk.toml ./
COPY public ./public

RUN rustup target add wasm32-unknown-unknown \
    && trunk build index.html

RUN cargo build --release --features server

FROM ubuntu:24.04 AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --create-home --shell /usr/sbin/nologin politech

WORKDIR /app

COPY --from=builder /app/target/release/politech /app/politech
COPY --from=builder /app/dist /app/dist
COPY --from=builder /app/public /app/public
COPY migrations /app/migrations

RUN chown -R politech:politech /app

ENV APP_HOST=0.0.0.0 \
    APP_PORT=3000

USER politech

EXPOSE 3000

CMD ["/app/politech"]
