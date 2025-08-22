pub struct Logger {}

impl Logger {
    pub fn log_connecting(uri: &str) {
        if cfg!(debug_assertions) {
            println!("Attempting to connect to {}", &uri);
        }
    }

    pub fn log_connected(uri: &str) {
        if cfg!(debug_assertions) {
            println!("Successfully connected to {}", &uri);
        }
    }

    pub fn log_subscribed() {
        if cfg!(debug_assertions) {
            println!("Sent: 'sub -e window_managed'");
        }
    }

    pub fn log_toggled() {
        if cfg!(debug_assertions) {
            println!("Sent: 'command toggle-tiling-direction'");
        }
    }

    pub fn log_tiling_error(msg: &str) {
        eprintln!("Error while listening wm events: {}", msg);
    }

    pub fn log_disconnected() {
        if cfg!(debug_assertions) {
            println!("WebSocket connection closed.");
        }
    }
}
