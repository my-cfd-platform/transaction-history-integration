use crate::{
    report_grpc::ReportOperationsHistoryInDateRangeGrpcResponseHistoryItem,
    transaction_history_integration_grpc::TransactionGrpcModel,
};

impl Into<TransactionGrpcModel> for ReportOperationsHistoryInDateRangeGrpcResponseHistoryItem {
    fn into(self) -> TransactionGrpcModel {
        TransactionGrpcModel {
            id: self.id,
            trader_id: self.trader_id,
            account_id: self.account_id,
            transaction_type: self.transaction_type,
            date_time: self.date_time_unix_micros,
            balance_delta: self.delta,
        }
    }
}
