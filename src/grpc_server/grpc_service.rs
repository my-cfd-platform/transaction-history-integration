use std::pin::Pin;

use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{
    transaction_history_integration_grpc::{
        transaction_history_integration_grpc_service_server::TransactionHistoryIntegrationGrpcService,
        GetTransactionHistoryGrpc, PingResponse, TransactionGrpcModel,
    },
    GrpcService,
};

#[tonic::async_trait]
impl TransactionHistoryIntegrationGrpcService for GrpcService {
    type GetTransactionHistoryStream = Pin<
        Box<
            dyn tonic::codegen::futures_core::Stream<
                    Item = Result<TransactionGrpcModel, tonic::Status>,
                > + Send
                + Sync
                + 'static,
        >,
    >;

    async fn get_transaction_history(
        &self,
        _: tonic::Request<GetTransactionHistoryGrpc>,
    ) -> Result<tonic::Response<Self::GetTransactionHistoryStream>, tonic::Status> {
        return my_grpc_extensions::grpc_server::send_vec_to_stream(vec![], |itm| itm).await;
    }
    async fn ping(
        &self,
        _: tonic::Request<()>,
    ) -> Result<tonic::Response<PingResponse>, tonic::Status> {
        return Ok(tonic::Response::new(PingResponse {
            service_name: "TRANSACTION_HISTORY_INTEGRATION".to_string(),
            date_time: DateTimeAsMicroseconds::now().unix_microseconds as u64,
        }));
    }
}
