use std::pin::Pin;

use my_telemetry::MyTelemetryContext;
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
        request: tonic::Request<GetTransactionHistoryGrpc>,
    ) -> Result<tonic::Response<Self::GetTransactionHistoryStream>, tonic::Status> {
        let request = request.into_inner();
        let history = self
            .app
            .report_grpc
            .get_in_date_range(
                &request.account_id,
                request.date_from_unix_ms,
                request.date_to_unix_ms,
                &MyTelemetryContext::new(),
            )
            .await;
        return my_grpc_extensions::grpc_server::send_vec_to_stream(history, |itm| itm.into())
            .await;
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
