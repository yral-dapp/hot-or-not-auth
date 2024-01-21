FROM scratch

WORKDIR /bin

COPY ./target/x86_64-unknown-linux-gnu/release/hot-or-not-auth .
COPY ./target/site ./site
COPY ./AuthConfig.toml .

ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_ENV="PROD"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
EXPOSE 3000

CMD ["./hot-or-not-auth"]
