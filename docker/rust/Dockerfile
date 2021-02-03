FROM rust:1.48

WORKDIR /so_searcher

COPY . .

RUN cargo build --release

RUN cargo install --path .

CMD [ "cargo", "run", "so_searcher" ]