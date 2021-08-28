FROM rust:slim

COPY . .

RUN apt-get update && apt-get install -y clang
RUN cargo install --path .

VOLUME [ "/var/lib/gossamer" ]

CMD ["gossamer"]