use std::net::SocketAddr;
use tonic::transport::Server;

use crate::transaction_history_integration_grpc::transaction_history_integration_grpc_service_server::TransactionHistoryIntegrationGrpcServiceServer;

#[derive(Clone)]
pub struct GrpcService {}

impl GrpcService {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn start_grpc_server(port: u16) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let service = GrpcService::new();

    println!("Listening to {:?} as grpc endpoint", addr);

    tokio::spawn(async move {
        Server::builder()
            .add_service(TransactionHistoryIntegrationGrpcServiceServer::new(
                service.clone(),
            ))
            .serve(addr)
            .await
    });
}
