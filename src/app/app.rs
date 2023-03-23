use std::sync::Arc;

use rust_extensions::AppStates;

use crate::{ReportGrpcClient, SettingsModel};

pub struct AppContext {
    pub report_grpc: Arc<ReportGrpcClient>,
    pub app_states: Arc<AppStates>,
}

impl AppContext {
    pub fn new(settings: &Arc<SettingsModel>) -> Self {
        Self {
            report_grpc: Arc::new(ReportGrpcClient::new(
                settings.report_grpc_service_url.clone(),
            )),
            app_states: Arc::new(AppStates::create_initialized()),
        }
    }
}
