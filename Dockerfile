FROM rust:1.56.0

WORKDIR /stormi

ENV STORMI_PORT=6345

COPY . .

RUN cargo build --release
RUN chmod +x target/release/stormi

EXPOSE ${STORMI_PORT} 

CMD [ "target/release/stormi" ]
