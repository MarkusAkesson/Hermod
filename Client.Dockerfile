FROM rust:1.49 as builder

RUN cargo install b3sum
RUN cargo install hyperfine

WORKDIR /usr/src/Hermod
COPY Cargo.toml .
COPY src/ src/.

RUN cargo install --path .

FROM debian:buster
COPY --from=builder /usr/local/cargo/bin/hermod /usr/local/bin/hermod
COPY --from=builder /usr/local/cargo/bin/b3sum /usr/local/bin/b3sum
COPY --from=builder /usr/local/cargo/bin/hyperfine /usr/local/bin/hyperfine
RUN mkdir ~/.hermod /output

RUN apt -y update
RUN apt -y install openssh-client

WORKDIR /sources

RUN fallocate -l 1G   large.file && \
    fallocate -l 500M medium.file && \
    fallocate -l 10K  small.file
COPY src/ src/.

WORKDIR /usr/src/test
COPY tools/run-test.sh .
COPY tools/run-bench.sh .

RUN chmod u+x run-test.sh
RUN chmod u+x run-bench.sh
