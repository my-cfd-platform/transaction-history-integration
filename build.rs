fn main() {
    tonic_build::compile_protos("proto/transaction_history_integration.proto").unwrap();
    tonic_build::compile_protos("proto/report_flows_grpc.proto").unwrap();
}
