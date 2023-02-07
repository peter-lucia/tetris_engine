FROM ubuntu:jammy

SHELL ["/bin/bash", "-c"]

RUN apt-get update -y && \
    apt-get install -y \
    nginx \
    curl \
    bash \
    net-tools \
    iproute2


WORKDIR /app

COPY target/x86_64-unknown-linux-gnu/release/rust_tetris .
#COPY target/x86_64-unknown-linux-gnu/debug/rust_tetris .
COPY tetris_frontend/sites-enabled /etc/nginx/sites-enabled/rust_tetris
COPY Rocket.toml Rocket.toml

COPY tetris_frontend/ tetris_frontend/


CMD service nginx restart

CMD ./rust_tetris & disown && nginx -g 'daemon off;'
