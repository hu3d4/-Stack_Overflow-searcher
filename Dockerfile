FROM rust:1.48

WORKDIR /so_searcher
COPY . .

# RUN apt-get update -y && apt-get upgrade -y
# RUN apt-get install -y postgresql postgresql-contrib

# RUN cargo install diesel_cli --no-default-features --features postgres

RUN cargo build --release
RUN cargo install --path .

CMD ["cargo", "run", "so_searcher"]
