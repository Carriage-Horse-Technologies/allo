use once_cell::sync::Lazy;

pub(crate) const ICON_SIZE: u32 = 128;

pub struct Config {
    pub(crate) location_provider_ws_url: &'static str,
    pub log_level: log::Level,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config {
    location_provider_ws_url: if cfg!(debug_assertions) {
        "ws://localhost/ws/example-user-id"
    } else {
        ""
    },
    log_level: if cfg!(debug_assertions) {
        log::Level::Trace
    } else {
        log::Level::Info
    },
});
