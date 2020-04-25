FROM rust:1.43 as builder

WORKDIR /usr/src/Hermod
COPY Cargo.toml .
COPY src/ src/.

RUN RUSTFLAGS='-C target-cpu=native' cargo install --path .

FROM debian:buster

COPY --from=builder /usr/local/cargo/bin/hermod /usr/local/bin/hermod

RUN mkdir ~/.hermod
RUN hermod server setup

RUN apt-get update && apt-get install -y openssh-server
RUN mkdir /var/run/sshd
RUN echo 'root:test' | chpasswd
RUN sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config

# SSH login fix. Otherwise user is kicked off after login
RUN sed 's@session\s*required\s*pam_loginuid.so@session optional pam_loginuid.so@g' -i /etc/pam.d/sshd

ENV NOTVISIBLE "in users profile"
RUN echo "export VISIBLE=now" >> /etc/profile

WORKDIR /usr/src/bench
COPY run-bench.sh .
RUN chmod u+x run-bench.sh

CMD ["/usr/src/bench/run-bench.sh", "server"]

