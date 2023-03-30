use crate::{
    report_grpc::ReportOperationHistoryItem,
    transaction_history_integration_grpc::TransactionGrpcModel,
};

impl Into<TransactionGrpcModel> for ReportOperationHistoryItem {
    fn into(self) -> TransactionGrpcModel {
        TransactionGrpcModel {
            id: self.id,
            trader_id: self.trader_id,
            account_id: self.account_id,
            transaction_type: self.transaction_type,
            process_id: self.process_id,
            balance_delta: self.delta,
            date: self.date,
            reference_transaction_id: self.reference_transaction_id,
            balance_after_operation: self.balance_after_operation,
        }
    }
}
