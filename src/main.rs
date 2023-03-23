use std::{sync::Arc};

use transaction_history_integration::{start_grpc_server, AppContext, SettingsReader};

#[tokio::main]
async fn main() {
    let settings_reader = SettingsReader::new("settings.json").await;
    let app = Arc::new(AppContext::new(&Arc::new(
        settings_reader.get_settings().await,
    )));
    start_grpc_server(app.clone(), 8888);

    app.app_states.wait_until_shutdown().await;
}
