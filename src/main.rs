#![windows_subsystem = "windows"]

use autotiling::{
    log::Logger,
    model::RootResponse,
    ws::{WebSocketStreamExt, connect_websocket_async},
};
use futures::StreamExt;
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut ws_stream = connect_websocket_async().await?;

    ws_stream.subscribe().await?;

    while let Some(result) = ws_stream.next().await {
        match result
            .map_err(|err| anyhow::Error::new(err))
            .map(get_message_text)
            .transpose()
            .and_then(|result| result.and_then(validate_tiling_size).transpose())
        {
            Some(Ok(_)) => {
                ws_stream.send_toggle_tiling().await?;
                Logger::log_toggled();
            }
            Some(Err(err)) => {
                Logger::log_tiling_error(&err.root_cause());
                break;
            }
            None => (),
        }
    }

    Logger::log_disconnected();
    Ok(())
}

fn get_message_text(msg: Message) -> Option<String> {
    if let Message::Text(text) = msg {
        Some(text.to_string())
    } else {
        None
    }
}

fn validate_tiling_size(json_text: String) -> anyhow::Result<Option<()>> {
    serde_json::from_str::<RootResponse>(json_text.as_str())
        .map_err(|e| anyhow::Error::new(e))
        .map(|json| {
            json.data
                .and_then(|data| data.managed_window)
                .and_then(|mw| mw.tiling_size)
                .and_then(|size| if size <= 0.5 { Some(()) } else { None })
        })
}
