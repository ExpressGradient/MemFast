FROM rust:latest

COPY . .

EXPOSE 3030

RUN cargo build --release

CMD ["cargo", "run", "--release"]
