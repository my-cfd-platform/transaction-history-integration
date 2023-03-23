use my_settings_reader::SettingsModel;
use serde::{Serialize, Deserialize};

#[derive(SettingsModel, Serialize, Deserialize, Clone)]
pub struct SettingsModel {
    #[serde(rename = "SeqConnString")]
    pub seq_conn_string: String,
    #[serde(rename = "MyTelemetry")]
    pub my_telemetry: String,
    #[serde(rename = "ReportGrpc")]
    pub report_grpc_service_url: String,
}
