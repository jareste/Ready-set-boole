FROM rust:latest

WORKDIR /usr/src/myapp

COPY . .

RUN rustc adder.rs -o adder
RUN rustc multiplier.rs -o multiplier
RUN rustc gray_code.rs -o gray_code
RUN rustc evaluation.rs -o evaluation

CMD ["sh", "-c", "./adder && ./multiplier && ./gray_code && ./evaluation"]

