FROM rust:1.48

WORKDIR /stack_overflow_searcher
COPY . .

RUN cargo build --release
RUN cargo install --path .

CMD ["stack_overflow_searcher"]
