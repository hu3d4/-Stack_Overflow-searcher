FROM rust:1.48

WORKDIR /stack_overflow_searcher
COPY . .

RUN apt-get update -y && apt-get upgrade -y
RUN apt-get install -y postgresql postgresql-contrib

RUN cargo install diesel_cli --no-default-features --features postgres

RUN cargo build --release
RUN cargo install --path .

CMD ["stack_overflow_searcher"]
