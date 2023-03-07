use once_cell::sync::Lazy;
use uuid::Uuid;

pub struct Config {
    pub(crate) location_provider_ws_url: &'static str,
    pub log_level: log::Level,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config {
    location_provider_ws_url: if cfg!(debug_assertions) {
        "ws://localhost/ws"
    } else {
        ""
    },
    log_level: if cfg!(debug_assertions) {
        log::Level::Trace
    } else {
        log::Level::Info
    },
});

pub static USER_ID: Lazy<String> = Lazy::new(|| Uuid::new_v4().to_string());
