use tokio_tungstenite::tungstenite::Message;

use crate::model::RootResponse;

pub fn get_message_text(msg: Message) -> Option<String> {
    if let Message::Text(bytes) = msg {
        Some(bytes.to_string())
    } else {
        None
    }
}

pub fn is_tiling_toggleable(json_text: &str) -> anyhow::Result<bool> {
    serde_json::from_str::<RootResponse>(json_text)
        .map_err(|e| anyhow::Error::new(e))
        .map(|json| {
            let size_result = json
                .data
                .and_then(|data| data.managed_window)
                .and_then(|mw| mw.tiling_size);
            match size_result {
                Some(size) if size < 0.5 => true,
                _ => false,
            }
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
    fn test_is_tiling_toggleable() {
        let json = r#"{"data":{"managedWindow":{"tilingSize":0.5}}}"#;
        assert_eq!(
            is_tiling_toggleable(json).unwrap(),
            true,
            "Expected tiling size to be toggleable"
        );

        let json = r#"{"data":{"managedWindow":{"tilingSize":0.501}}}"#;
        assert_eq!(
            is_tiling_toggleable(json).unwrap(),
            false,
            "Expected tiling size to be not toggleable"
        );
    }
}
