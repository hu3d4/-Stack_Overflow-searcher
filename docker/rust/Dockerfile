FROM rust:1.48

WORKDIR /qiita_searcher

COPY . .

ENV USER root

RUN apt-get update -y && apt-get upgrade -y
RUN apt-get install -y postgresql postgresql-contrib

RUN cargo install --path .

RUN cargo install diesel_cli --no-default-features --features postgres

RUN cargo build --release

CMD [ "cargo", "run", "qiita_searcher" ]