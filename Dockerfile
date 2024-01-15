FROM scratch

WORKDIR /bin

COPY ./target/x86_64-unknown-linux-gnu/release/hot-or-not-auth .

EXPOSE 8080

CMD ["./hot-or-not-auth"]
