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
        "wss://location-provider.yukinissie.com/ws"
    },
    log_level: if cfg!(debug_assertions) {
        log::Level::Trace
    } else {
        log::Level::Info
    },
});

pub const GITHUB_USERNAME_KEY: &'static str = "github_user_name";
pub const MOVE_SPEED_MS: u32 = 500;
pub const MOVING_DISTANCE: f64 = 50.0;
pub const CHARA_SIZE: u32 = 64;
pub const CHARA_OFFSET: u32 = CHARA_SIZE / 2;
pub const SCROLL_PX: f64 = 10.;
// w-[2000px] h-[2000px]
pub const WORLD_SIZE: (u32, u32) = (2000, 2000);
pub static WORLD_SIZE_CLASS_W: Lazy<String> = Lazy::new(|| format!("w-[{}px]", WORLD_SIZE.0));
pub static WORLD_SIZE_CLASS_H: Lazy<String> = Lazy::new(|| format!("h-[{}px]", WORLD_SIZE.1));
