FROM lukemathwalker/cargo-chef:latest-rust-bookworm AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as prisma
COPY .cargo /app/.cargo
COPY prisma-cli /app
COPY prisma /app/prisma
RUN cargo prisma generate

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
COPY --from=prisma /app/src/db.rs /app/src/db.rs
RUN cargo build --release --bin dyson-api

FROM debian:bookworm-slim as runtime
RUN apt-get update && apt-get install -y openssl && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/dyson-api /usr/local/bin

EXPOSE 8000
ENTRYPOINT ["/usr/local/bin/dyson-api"]

HEALTHCHECK --interval=5m \
	CMD curl -f http://localhost:8000/ || exit 1
