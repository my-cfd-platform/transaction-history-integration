mod grpc_server;
mod grpc_client;
mod app;
mod settings;

pub mod transaction_history_integration_grpc {
    tonic::include_proto!("transaction_history_integration");
}

pub mod report_grpc {
    tonic::include_proto!("report_grpc");
}


pub use grpc_server::*;
pub use grpc_client::*;
pub use app::*;
pub use settings::*;