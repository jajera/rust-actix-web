FROM docker.io/rust:latest

WORKDIR /app/

ADD [".", "/app/"]

ENTRYPOINT ["cargo", "run"]