FROM rust:1.80.1 as builder

WORKDIR /usr/src/pbudget

COPY . .

RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/pbudget /usr/local/bin/pbudget

EXPOSE 8080

CMD ["./target/release/pbudget"]