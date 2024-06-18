FROM rust:latest

WORKDIR /usr/src/myapp

COPY . .

RUN rustc adder.rs -o adder

CMD ["sh", "-c", "./adder"]
