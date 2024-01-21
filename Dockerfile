FROM rust:1.75-buster

WORKDIR /app

COPY . .

RUN cargo install

EXPOSE 8000

CMD [ "cargo", "run" ]