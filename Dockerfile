FROM rust:latest

WORKDIR /usr/src/myapp

COPY . .

RUN rustc adder.rs -o adder
RUN rustc multiplier.rs -o multiplier

CMD ["sh", "-c", "./adder && ./multiplier"]
