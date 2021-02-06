# FROM rust:1.48

# WORKDIR /stack_overflow_searcher
# COPY . .
# RUN cargo build --release
# RUN cargo install --path .

# CMD ["stack_overflow_searcher"]

FROM rust:1.48

WORKDIR /stack_overflow_searcher
COPY Cargo.toml Cargo.toml
COPY ./src ./src
COPY ./templates ./templates

COPY ./migrations ./migrations
COPY diesel.toml diesel.toml

ENV DATABASE_URL=postgres://so_searcher:so_searcher_password@0.0.0.0:5433/so_searcher

# COPY . .

RUN cargo build --release
RUN cargo install --path .

CMD ["stack_overflow_searcher"]
