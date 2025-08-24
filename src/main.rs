#![windows_subsystem = "windows"]

use autotiling::{
    log::Logger,
    util::{get_message_text, validate_tiling_size},
    ws::{WebSocketStreamExt, connect_websocket_async},
};
use futures::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut ws_stream = connect_websocket_async().await?;

    ws_stream.subscribe().await?;

    while let Some(result) = ws_stream.next().await {
        match result
            .map_err(|err| anyhow::Error::new(err))
            .map(get_message_text)
            .transpose()
            .and_then(|result| {
                result
                    .and_then(|str| validate_tiling_size(str.as_str()))
                    .transpose()
            }) {
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
