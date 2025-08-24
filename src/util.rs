use tokio_tungstenite::tungstenite::Message;

use crate::model::RootResponse;

pub fn get_message_text(msg: Message) -> Option<String> {
    if let Message::Text(bytes) = msg {
        Some(bytes.to_string())
    } else {
        None
    }
}

pub fn validate_tiling_size(json_text: &str) -> anyhow::Result<Option<()>> {
    serde_json::from_str::<RootResponse>(json_text)
        .map_err(|e| anyhow::Error::new(e))
        .map(|json| {
            json.data
                .and_then(|data| data.managed_window)
                .and_then(|mw| mw.tiling_size)
                .and_then(|size| if size <= 0.5 { Some(()) } else { None })
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_message_text() {
        let msg = Message::Text("Hello, world!".into());
        assert_eq!(
            get_message_text(msg),
            Some("Hello, world!".into()),
            "Expected text message to be extracted"
        );

        let msg = Message::Binary(vec![1, 2, 3].into());
        assert_eq!(
            get_message_text(msg),
            None,
            "Expected binary message to be ignored"
        );
    }

    #[test]
    fn test_validate_tiling_size() {
        let json = r#"{"data":{"managedWindow":{"tilingSize":0.5}}}"#;
        assert_eq!(
            validate_tiling_size(json).unwrap(),
            Some(()),
            "Expected tiling size to be valid"
        );

        let json = r#"{"data":{"managedWindow":{"tilingSize":0.501}}}"#;
        assert_eq!(
            validate_tiling_size(json).unwrap(),
            None,
            "Expected tiling size to be invalid"
        );
    }
}
