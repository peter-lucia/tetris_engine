FROM ubuntu:jammy

SHELL ["/bin/bash", "-c"]

RUN apt-get update -y && \
    apt-get install -y \
    curl \
    bash \
    net-tools \
    iproute2


WORKDIR /app

RUN useradd spock
RUN chown -R spock:spock /app
USER spock

COPY target/x86_64-unknown-linux-gnu/rust_tetris .

CMD rust_tetris