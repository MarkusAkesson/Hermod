FROM rust:1.42 as builder

WORKDIR /usr/src/Hermod
COPY Cargo.toml .
COPY src/ src/.

RUN RUSTFLAGS='-C target-cpu=native' cargo install --path .
RUN cargo install b3sum
RUN cargo install hyperfine

FROM debian:buster
COPY --from=builder /usr/local/cargo/bin/hermod /usr/local/bin/hermod
COPY --from=builder /usr/local/cargo/bin/b3sum /usr/local/bin/b3sum
COPY --from=builder /usr/local/cargo/bin/hyperfine /usr/local/bin/hyperfine
RUN mkdir ~/.hermod /output

RUN apt-get update && apt-get install -y openssh-server

WORKDIR /sources

RUN fallocate -l 1G   large.file && \
    fallocate -l 500M medium.file && \
    fallocate -l 10K  small.file

COPY src/ src/.

WORKDIR /usr/src/bench
COPY run-bench.sh .
RUN chmod u+x run-bench.sh

CMD ["/usr/src/bench/run-bench.sh"]
