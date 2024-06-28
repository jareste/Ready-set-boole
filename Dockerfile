FROM rust:latest

WORKDIR /usr/src/myapp

COPY . .

RUN rustc adder.rs -o adder
RUN rustc multiplier.rs -o multiplier
RUN rustc gray_code.rs -o gray_code
RUN rustc evaluation.rs -o evaluation
RUN rustc truth_table.rs -o truth_table
RUN rustc negation_normal_form.rs -o negation_normal_form
RUN rustc conjuntive_normal_form.rs -o conjuntive_normal_form
RUN rustc sat.rs -o sat
RUN rustc powerset.rs -o powerset
RUN rustc set_evaluation.rs -o set_evaluation
RUN rustc curve.rs -o curve
RUN rustc inverse_function.rs -o inverse_function

CMD ["sh", "-c", "./adder && ./multiplier && ./gray_code && ./evaluation && ./truth_table && ./negation_normal_form && ./conjuntive_normal_form && ./sat && ./powerset && ./set_evaluation && ./curve && ./inverse_function"]

