FROM ubuntu:latest

COPY /target/x86_64-unknown-linux-gnu/release/memfast .

EXPOSE 3030

ENTRYPOINT ["./memfast"]
