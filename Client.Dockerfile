FROM rust:1.41 as builder

WORKDIR /usr/src/Hermod
COPY Cargo.toml .
COPY src/ src/.

RUN cargo install --path .

FROM debian:buster
COPY --from=builder /usr/local/cargo/bin/hermod /usr/local/bin/hermod
RUN mkdir ~/.hermod

VOLUME /sources

WORKDIR /sources

RUN fallocate -l 1G   large.file && \
    fallocate -l 500M medium.file && \
    fallocate -l 10K  small.file

WORKDIR /usr/src/test
COPY run-test.sh .
RUN chmod u+x run-test.sh

CMD ["/usr/src/test/run-test.sh"]
