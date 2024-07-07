FROM docker.io/rust:latest

WORKDIR /app/

ADD [".", "/app/"]

EXPOSE 8080

ENTRYPOINT ["cargo", "run"]