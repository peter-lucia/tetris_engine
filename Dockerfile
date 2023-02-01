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
COPY tetris_frontend/ /var/www/tetris
COPY tetris_frontend/sites-enabled /etc/nginx/sites-enabled/rust_tetris
COPY tetris_frontend/*.html /usr/share/nginx/html/
COPY tetris_frontend/css/*.css /usr/share/nginx/html/
COPY tetris_frontend/js/*.js /usr/share/nginx/html/


CMD service nginx restart

#RUN useradd spock
#RUN chown -R spock:spock /app
#USER spock

CMD ./rust_tetris &&
CMD nginx -g 'daemon off;'
