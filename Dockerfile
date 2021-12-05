FROM rust:1.56.0

ENV STORMI_PORT=6345
WORKDIR /stormi
COPY . .

RUN cargo build --release
RUN chmod +x target/release/stormi
RUN mv target/release/stormi /tmp/stormi
RUN rm -rf *
RUN mv /tmp/stormi ./stormi

EXPOSE ${STORMI_PORT} 

CMD [ "./stormi" ]
