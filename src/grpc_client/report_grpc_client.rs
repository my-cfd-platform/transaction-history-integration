use std::{sync::Arc, time::Duration};

use my_grpc_extensions::{GrpcChannel, GrpcClientInterceptor, GrpcClientSettings};
use my_telemetry::MyTelemetryContext;
use tonic::{codegen::InterceptedService, transport::Channel};

use crate::report_grpc::{
    report_grpc_service_client::ReportGrpcServiceClient, ReportOperationHistoryItem, ReportFlowsOperationsGetInDateRangeGrpcRequest,
};

pub const TRANSACTION_GRPC_SERVICE_NAME: &str = "transaction_history_integration";

pub struct ReportGrpcClient {
    grpc_channel: GrpcChannel,
}

pub struct CandleDateKey(String);

#[tonic::async_trait]
impl GrpcClientSettings for CandleDateKey {
    async fn get_grpc_url(&self, _: &'static str) -> String {
        return self.0.clone();
    }
}

impl ReportGrpcClient {
    pub fn new(get_grpc_address: String) -> Self {
        Self {
            grpc_channel: GrpcChannel::new(
                Arc::new(CandleDateKey(get_grpc_address)),
                TRANSACTION_GRPC_SERVICE_NAME,
                Duration::from_secs(10),
            ),
        }
    }

    async fn create_grpc_service(
        &self,
        ctx: &MyTelemetryContext,
    ) -> ReportGrpcServiceClient<InterceptedService<Channel, GrpcClientInterceptor>> {
        let client: ReportGrpcServiceClient<InterceptedService<Channel, GrpcClientInterceptor>> =
            ReportGrpcServiceClient::with_interceptor(
                self.grpc_channel.get_channel().await.unwrap(),
                GrpcClientInterceptor::new(ctx.clone()),
            );

        client
    }

    pub async fn get_in_date_range(
        &self,
        account_id: &str,
        from: Option<u64>,
        to: Option<u64>,
        ctx: &MyTelemetryContext,
    ) -> Vec<ReportOperationHistoryItem> {
        let mut grpc_client = self.create_grpc_service(ctx).await;

        let request = ReportFlowsOperationsGetInDateRangeGrpcRequest {
            account_id: account_id.to_string(),
            date_from: from,
            date_to: to,
        };

        let result = grpc_client
            .get_operations_history_in_date_range(request)
            .await
            .unwrap()
            .into_inner();

        let result = my_grpc_extensions::read_grpc_stream::as_vec(result, Duration::from_secs(20))
        .await
        .unwrap();

        return match result{
            Some(src) => src,
            None => vec![],
        }
    }
}
