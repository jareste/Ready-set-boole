FROM rust:latest

WORKDIR /usr/src/myapp

COPY . .

RUN rustc adder.rs -o adder
RUN rustc multiplier.rs -o multiplier
RUN rustc gray_code.rs -o gray_code
RUN rustc evaluation.rs -o evaluation
RUN rustc truth_table.rs -o truth_table
# RUN rustc negation_normal_form.rs -o negation_normal_form NOOK
RUN rustc sat.rs -o sat

CMD ["sh", "-c", "./adder && ./multiplier && ./gray_code && ./evaluation && ./truth_table && ./sat"]

