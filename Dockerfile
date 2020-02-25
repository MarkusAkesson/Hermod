FROM rust:1.41 as builder

WORKDIR /usr/src/Hermod
COPY Cargo.toml .
COPY src/ src/.

RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/hermod /usr/local/bin/hermod
RUN mkdir ~/.hermod
RUN hermod server setup

CMD ["hermod", "server", "--no-daemon"]

EXPOSE 4444
