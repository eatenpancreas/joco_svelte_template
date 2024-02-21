FROM rust:latest
RUN cargo install sqlx-cli

WORKDIR /app
COPY ./api /app/api
COPY ./api-lib /app/api-lib
COPY ./api-proc /app/api-proc
COPY ./migrations /app/migrations
COPY ./Cargo.toml /app/Cargo.toml
COPY ./Cargo.lock /app/Cargo.lock
COPY .env.production /app/.env

RUN cargo sqlx database create
RUN cargo sqlx migrate run
RUN cargo sqlx prepare --workspace
RUN cargo build --release
EXPOSE 8080
CMD ["./target/release/api"]
