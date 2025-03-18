use std::fs::File;
use std::io::{Error, ErrorKind, Write};
use std::path::Path;
use serde::Serialize;
use crate::error::RelicWizardError;
use crate::error::RelicWizardError::PrimaryMonitorInaccessible;

#[derive(Serialize)]
pub struct Config {
    pub scan_settings: ScanSettings,
    pub general_settings: GeneralSettings
}

#[derive(Serialize)]
pub struct GeneralSettings {
    pub screen_width: u32,
    pub screen_height: u32,
    pub primary_monitor_name: String,
    pub primary_monitor_id: u32
}

#[derive(Serialize)]
pub struct ScanSettings {
    pub maximum_total_scan_duration_sec: u16,
    pub relic_switch_time_ms: u16
}

fn get_primary_monitor() -> Result<xcap::Monitor, RelicWizardError> {
    let monitors = xcap::Monitor::all();
    
    if monitors.is_err() {
        return Err(PrimaryMonitorInaccessible(Error::new(ErrorKind::Other, monitors.err().unwrap())));
    }

    for monitor in monitors.unwrap() {
        if monitor.is_primary() {
            return Ok(monitor);
        }
    }

    Err(PrimaryMonitorInaccessible(Error::new(ErrorKind::NotFound, "No primary monitor found")))
}

pub fn write_default_config(path: &Path) -> Option<Error> {

    let monitor = match get_primary_monitor() {
        Ok(monitor) => monitor,
        Err(e) => return Some(Error::new(ErrorKind::Other, e))
    };

    let conf = Config {
        general_settings: GeneralSettings {
            screen_height: monitor.height(),
            screen_width: monitor.width(),
            primary_monitor_name: String::from(monitor.name()),
            primary_monitor_id: monitor.id(),
        },
        scan_settings: ScanSettings {
            maximum_total_scan_duration_sec: 0,
            relic_switch_time_ms: 1000
        }
    };

    let content = toml::to_string_pretty(&conf);
    let file = File::create(path);

    if file.is_ok() && content.is_ok() {
        let res = file.unwrap().write_all(content.unwrap().as_bytes());

        if res.is_err() {
            return Some(Error::from(ErrorKind::DirectoryNotEmpty));
        }
    }
    None
}