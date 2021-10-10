FROM rustlang/rust:nightly-alpine3.12 as compile-rust-service
WORKDIR /app
COPY . ./
RUN apk add musl-dev protoc build-base && gcc resources/c/shell.c -o resources/c/shell && cargo build --release

FROM alpine:latest
LABEL Maintainer="Moussa Haidous <moussa@haidousm.com>"
LABEL Description="a (basic) UNIX shell API"
LABEL version="1.0.0"
WORKDIR /app
COPY --from=compile-rust-service --chown=nobody /app/target/release/unix-shell-api ./
COPY --from=compile-rust-service --chown=nobody /app/resources ./resources
USER nobody
ENV ROCKET_ADDRESS 0.0.0.0
CMD ["./unix-shell-api"]
EXPOSE 8000