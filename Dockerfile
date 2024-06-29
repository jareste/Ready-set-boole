FROM rust:latest

WORKDIR /usr/src/myapp

COPY . .

RUN rustc main.rs -o main

CMD ["sh", "-c", "./main"]