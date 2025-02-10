use serde::Serialize;

#[derive(Serialize)]
pub struct Config {
    pub scan_settings: ScanSettings
}

#[derive(Serialize)]
pub struct ScanSettings {
    pub show_console: bool,
    pub show_debug: bool
}