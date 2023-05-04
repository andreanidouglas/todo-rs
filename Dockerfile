FROM lukemathwalker/cargo-chef:latest-rust-latest as chef 
WORKDIR /app
RUN apt update && apt install lld clang -y

# Cargo chef prepare (cache all cargo dependencies)
FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Cargo chef cook and build (get cached dependencies and build binary)
FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin todo-rust

# Copy binary to basic image and prepare it for execution
FROM debian:bullseye as runtime
WORKDIR /app
RUN apt-get update && \
    apt-get install -y --no-install-recommends openssl ca-certificates && \
    apt-get autoremove -y && \
    apt-get clean -y && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/todo-rust todo-rust
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT [ "./todo-rust" ]
