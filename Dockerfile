FROM rust:1.84 as builder
WORKDIR /usr/src/receipt-processor
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN adduser processor
USER processor
COPY --from=builder /usr/local/cargo/bin/receipt-processor /usr/local/bin/receipt-processor
EXPOSE 3030
CMD ["receipt-processor"]
