FROM rust:slim

COPY . /data/gossamer

RUN apt-get update && apt-get install -y clang
RUN cargo install --path .

VOLUME [ "/var/lib/gossamer" ]

CMD ["gossamer"]