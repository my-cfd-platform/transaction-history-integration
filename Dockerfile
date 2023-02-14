FROM rust:slim
COPY ./target/release/transaction-history-integration ./target/release/transaction-history-integration
ENTRYPOINT ["./target/release/transaction-history-integration"]