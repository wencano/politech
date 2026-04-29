# syntax=docker/dockerfile:1
# Target: Ubuntu 24.04 LTS (x86_64) — matches Dokploy host

FROM ubuntu:24.04 AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        build-essential \
        pkg-config \
        libssl-dev \
        nodejs \
        npm \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --default-toolchain none

ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app

COPY rust-toolchain.toml Cargo.toml Cargo.lock ./
RUN rustup toolchain install

COPY src ./src
COPY migrations ./migrations
COPY index.html Trunk.toml ./
COPY web ./web
COPY public ./public
COPY package.json ./

RUN npm install \
    && npm run css:build \
    && cargo install trunk --locked \
    && rustup target add wasm32-unknown-unknown \
    && NO_COLOR=1 trunk build index.html \
    && cargo build --release --features server

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
