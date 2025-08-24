use futures::SinkExt;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async,
    tungstenite::{self, Message, Utf8Bytes},
};

use crate::log::Logger;

const URI: &str = "ws://localhost:6123";

const TOGGLE_MESSAGE: Message =
    Message::Text(Utf8Bytes::from_static("command toggle-tiling-direction"));

pub async fn connect_websocket_async()
-> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, tungstenite::error::Error> {
    Logger::log_connecting(URI);
    let (ws_stream, _) = connect_async(URI).await?;
    Logger::log_connected(URI);
    Ok(ws_stream)
}

pub trait WebSocketStreamExt {
    fn subscribe(&mut self) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
    fn send_toggle_tiling(
        &mut self,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
}

impl WebSocketStreamExt for WebSocketStream<MaybeTlsStream<TcpStream>> {
    async fn subscribe(&mut self) -> anyhow::Result<()> {
        let message: Message = Message::Text(Utf8Bytes::from_static("sub -e window_managed"));
        self.send(message).await?;
        Logger::log_subscribed();
        Ok(())
    }

    async fn send_toggle_tiling(&mut self) -> anyhow::Result<()> {
        self.send(TOGGLE_MESSAGE).await?;
        Logger::log_toggled();
        Ok(())
    }
}
