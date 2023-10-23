FROM rust:1.73 as builder
WORKDIR /usr/src/deadjokes-api
COPY . .
RUN cargo install --bin deadjokes-api --path .

FROM ubuntu:22.04
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/deadjokes-api /usr/local/bin/deadjokes-api
CMD ["deadjokes-api"]
