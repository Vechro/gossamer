# --- Builder image ---
FROM rust:slim as builder

WORKDIR /gossamer

COPY . .

RUN apt-get update && apt-get install -y clang
RUN cargo build --release

# --- Final image ---
FROM gcr.io/distroless/cc

WORKDIR /gossamer

COPY --from=builder /gossamer/target/release/gossamer .
COPY --from=builder /gossamer/static ./static

VOLUME ["/var/lib/gossamer"]

CMD ["/gossamer/gossamer"]