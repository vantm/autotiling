#![windows_subsystem = "windows"]

use std::error::Error;

use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async,
    tungstenite::{Message, Utf8Bytes},
};

#[derive(Debug, Deserialize)]
struct RootResponse {
    data: Option<Data>,
}

#[derive(Debug, Deserialize)]
struct Data {
    #[serde(rename = "managedWindow")]
    managed_window: Option<ManagedWindow>,
}

#[derive(Debug, Deserialize)]
struct ManagedWindow {
    #[serde(rename = "tilingSize")]
    tiling_size: Option<f64>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uri = "ws://localhost:6123";

    Logger::log_connecting(uri);

    let (mut ws_stream, _) = connect_async(uri).await?;

    Logger::log_connected(uri);

    let sub_message = Message::Text("sub -e window_managed".into());
    ws_stream.send(sub_message).await?;

    Logger::log_subscribed();

    while let Some(msg_result) = ws_stream.next().await {
        match msg_result {
            Ok(msg) => {
                if let Message::Text(text) = msg {
                    match serde_json::from_str::<RootResponse>(&text) {
                        Ok(json_response) => {
                            let size_percentage = json_response
                                .data
                                .and_then(|data| data.managed_window)
                                .and_then(|mw| mw.tiling_size);

                            if let Some(size) = size_percentage {
                                if size <= 0.5 {
                                    ws_stream.send_toggle_tiling().await?;
                                    Logger::log_toggled(size);
                                }
                            }
                        }
                        Err(e) => {
                            Logger::log_tiling_error(&e, &text);
                        }
                    }
                }
            }
            Err(e) => {
                Logger::log_websocket_error(&e);
                break;
            }
        }
    }

    Logger::log_disconnected();
    Ok(())
}

trait WebSocketStreamExt {
    async fn send_toggle_tiling(&mut self) -> Result<(), tokio_tungstenite::tungstenite::Error>;
}

const COMMAND_MESSAGE: Message =
    Message::Text(Utf8Bytes::from_static("command toggle-tiling-direction"));

impl WebSocketStreamExt for WebSocketStream<MaybeTlsStream<TcpStream>> {
    async fn send_toggle_tiling(&mut self) -> Result<(), tokio_tungstenite::tungstenite::Error> {
        self.send(COMMAND_MESSAGE).await
    }
}

struct Logger {}

impl Logger {
    fn log_connecting(uri: &str) {
        println!("Attempting to connect to {}", uri);
    }

    fn log_connected(uri: &str) {
        println!("Successfully connected to {}", uri);
    }

    fn log_subscribed() {
        println!("Sent: 'sub -e window_managed'");
    }

    fn log_toggled(size: f64) {
        println!(
            "Sent: 'command toggle-tiling-direction' (tilingSize: {})",
            size
        );
    }

    fn log_tiling_error(e: &dyn Error, text: &str) {
        eprintln!("Failed to parse JSON: {} (Original message: {})", e, text);
    }

    fn log_websocket_error(e: &dyn Error) {
        eprintln!("WebSocket error: {}", e);
    }

    fn log_disconnected() {
        println!("WebSocket connection closed.");
    }
}
