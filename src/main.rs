use exhibition_lp::{app::App, settings::CONFIG};

fn main() {
    wasm_logger::init(wasm_logger::Config::new(CONFIG.log_level));
    yew::Renderer::<App>::new().render();
}
