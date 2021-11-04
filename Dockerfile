FROM rust:1.56.0

WORKDIR /stormi

COPY . .

RUN cargo build --release
RUN chmod +x target/release/stormi

CMD [ "target/release/stormi" ]
