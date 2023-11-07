use axum::{Extension, Json};
use serde::{Deserialize, Serialize};

pub async fn healthcheck(
    Extension(address): Extension<[u8; 4]>,
    Extension(port): Extension<u16>,
) -> Json<ServerInfo> {
    let server_info = ServerInfo { address, port };

    Json(server_info)
}

#[derive(Serialize, Deserialize)]
pub struct ServerInfo {
    pub address: [u8; 4],
    pub port: u16,
}
