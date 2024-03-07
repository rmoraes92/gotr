use iced::Application;
use gotr::app::App;
use iced::{
    Settings,
    // Size,
    window::Settings as WindowSettings
};

static INITIAL_WIDTH: u32 = 1024;
static INITIAL_HEIGHT: u32 = 768;

pub fn main() -> iced::Result {
    //#[cfg(not(target_arch = "wasm32"))]
    //tracing_subscriber::fmt::init();
    App::run(Settings {
        window: WindowSettings {
            size: (INITIAL_WIDTH, INITIAL_HEIGHT),
            ..WindowSettings::default()
        },
        ..Settings::default()
    })
}
