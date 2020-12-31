FROM rust:1.48

WORKDIR /qiita_searcher

ENV USER root

RUN apt-get update -y && apt-get upgrade -y
RUN apt-get install -y postgresql postgresql-contrib

RUN cargo install diesel_cli --no-default-features --features postgres