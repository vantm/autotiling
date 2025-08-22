use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RootResponse {
    pub data: Option<Data>,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    #[serde(rename = "managedWindow")]
    pub managed_window: Option<ManagedWindow>,
}

#[derive(Debug, Deserialize)]
pub struct ManagedWindow {
    #[serde(rename = "tilingSize")]
    pub tiling_size: Option<f64>,
}
