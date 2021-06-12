FROM rust:1.49 as builder

WORKDIR /usr/src/Hermod
COPY Cargo.toml .
COPY src/ src/.

RUN apt -y update
RUN apt -y install clang
RUN cargo install --path .
RUN cargo install b3sum

FROM debian:buster
COPY --from=builder /usr/local/cargo/bin/hermod /usr/local/bin/hermod
COPY --from=builder /usr/local/cargo/bin/b3sum /usr/local/bin/b3sum

RUN apt -y update
RUN apt -y install openssh-client openssh-server openssh-sftp-server

RUN mkdir ~/.hermod
RUN hermod server setup

CMD ["hermod", "server", "--no-daemon"]
