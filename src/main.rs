#![windows_subsystem = "windows"]

use autotiling::{
    log::Logger,
    util::{get_message_text, is_tiling_toggleable},
    ws::{WebSocketStreamExt, connect_websocket_async},
};
use futures::StreamExt;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut ws_stream = connect_websocket_async().await?;
    ws_stream.subscribe().await?;

    loop {
        if let Err(err) = listen_message(&mut ws_stream).await {
            Logger::log_tiling_error(&err);
            break;
        }
    }

    Logger::log_disconnected();

    Ok(())
}

async fn listen_message(
    ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
) -> anyhow::Result<()> {
    if let Some(result) = ws_stream.next().await {
        if let Some(json_text) = result
            .map(get_message_text)
            .map_err(|e| anyhow::Error::new(e))?
        {
            if let Ok(true) = is_tiling_toggleable(&json_text) {
                ws_stream.send_toggle_tiling().await?;
            }
        }
    }

    Ok(())
}
