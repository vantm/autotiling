use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio_tungstenite::{connect_async, tungstenite::Message};

// Define the Rust data structures that mirror the expected JSON response.
// This allows `serde_json` to automatically parse the incoming JSON into
// these types. The `Option` types are crucial for handling cases where
// keys might be missing or their values might be null, similar to Python's
// `try-except KeyError` or checking for `None`.

#[derive(Debug, Deserialize)]
struct RootResponse {
    // The 'data' field might be missing or null in some responses,
    // so we wrap it in an `Option`.
    data: Option<Data>,
}

#[derive(Debug, Deserialize)]
struct Data {
    // The JSON key "managedWindow" is mapped to the Rust field `managed_window`.
    // This field might also be missing or null.
    #[serde(rename = "managedWindow")]
    managed_window: Option<ManagedWindow>,
}

#[derive(Debug, Deserialize)]
struct ManagedWindow {
    // The JSON key "tilingSize" is mapped to the Rust field `tiling_size`.
    // This field can be a number (f64) or null.
    #[serde(rename = "tilingSize")]
    tiling_size: Option<f64>,
}

// The `#[tokio::main]` attribute sets up the Tokio runtime for asynchronous operations.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uri = "ws://localhost:6123";

    println!("Attempting to connect to {}", uri);

    // Establish a WebSocket connection. `connect_async` returns a tuple
    // containing the WebSocket stream and the HTTP response (which we ignore here).
    let (mut ws_stream, _) = connect_async(uri).await?;
    println!("Successfully connected to {}", uri);

    // Send the initial subscription message.
    let sub_message = Message::Text("sub -e window_managed".into());
    ws_stream.send(sub_message).await?;
    println!("Sent: 'sub -e window_managed'");

    // Enter an infinite loop to receive messages, similar to Python's `while True`.
    while let Some(msg_result) = ws_stream.next().await {
        match msg_result {
            Ok(msg) => {
                // We only care about text messages, as per the Python code.
                if let Message::Text(text) = msg {
                    // Uncomment the line below for debugging received raw JSON messages:
                    // println!("Received raw: {}", text);

                    // Attempt to parse the received text as JSON into our `RootResponse` structure.
                    match serde_json::from_str::<RootResponse>(&text) {
                        Ok(json_response) => {
                            // Safely access the nested `tilingSize` field.
                            // `and_then` is used to chain `Option` operations,
                            // allowing us to gracefully handle missing intermediate keys.
                            let size_percentage = json_response
                                .data
                                .and_then(|data| data.managed_window)
                                .and_then(|mw| mw.tiling_size);

                            // Check if `tiling_size` was present and not null.
                            // This `if let Some(size) = size_percentage` handles
                            // `if sizePercentage is None: continue` from Python.
                            if let Some(size) = size_percentage {
                                // Apply the core logic: if `tilingSize` is less than or equal to 0.5.
                                if size <= 0.5 {
                                    let command_message =
                                        Message::Text("command toggle-tiling-direction".into());
                                    ws_stream.send(command_message).await?;
                                    println!(
                                        "Sent: 'command toggle-tiling-direction' (tilingSize: {})",
                                        size
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            // This block handles JSON parsing errors, including cases where
                            // expected keys are missing (similar to Python's `KeyError`).
                            // The Python code silently `pass`es on `KeyError`, so here we just log
                            // the error without stopping the program.
                            eprintln!("Failed to parse JSON: {} (Original message: {})", e, text);
                        }
                    }
                }
                // Other message types (Binary, Ping, Pong, Close) are ignored,
                // consistent with the Python example.
            }
            Err(e) => {
                // If there's an error receiving a message from the WebSocket,
                // print the error and break out of the loop, closing the connection.
                eprintln!("WebSocket error: {}", e);
                break;
            }
        }
    }

    println!("WebSocket connection closed.");
    Ok(()) // Indicate successful execution of the main function
}
