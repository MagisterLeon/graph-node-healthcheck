FROM rustlang/rust:nightly-buster-slim as builder

RUN apt-get update -y && apt-get install libssl-dev pkg-config -y
WORKDIR /app
COPY . .
RUN cargo install --path .


FROM debian:buster-slim as runner

RUN apt-get update -y && apt-get install libssl-dev pkg-config -y

COPY --from=builder /usr/local/cargo/bin/graph-node-healthcheck /usr/local/bin/graph-node-healthcheck
ENV ROCKET_PORT=7010
EXPOSE 7010
CMD ["graph-node-healthcheck"]