mod grpc_server;

pub mod transaction_history_integration_grpc {
    tonic::include_proto!("transaction_history_integration");
}

pub use grpc_server::*;