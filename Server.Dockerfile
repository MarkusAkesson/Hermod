FROM rust:1.41 as builder

WORKDIR /usr/src/Hermod
COPY Cargo.toml .
COPY src/ src/.

RUN cargo install --path .

FROM debian:buster
COPY --from=builder /usr/local/cargo/bin/hermod /usr/local/bin/hermod
RUN mkdir ~/.hermod
VOLUME /output

ONBUILD RUN hermod server setup

CMD ["hermod", "server", "--no-daemon"]
