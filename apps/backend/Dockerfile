# Stage 1 — install cargo-chef
FROM rust:1.87-slim AS chef
RUN cargo install cargo-chef
WORKDIR /app

# Stage 2 — generate dependency recipe
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3 — build dependencies (this layer gets cached)
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
RUN cargo chef cook --release --recipe-path recipe.json

# Build the actual app
COPY . .
RUN cargo build --release --bin backend

# Stage 4 — minimal runtime image
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/backend ./backend
EXPOSE 3000
CMD ["./backend"]
