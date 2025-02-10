use std::fs::File;
use std::path::Path;
use std::io::{Error, ErrorKind, Write};
use crate::config;
use crate::config::ScanSettings;

fn dir_exists(path: &Path) -> Result<bool, Error> {
    let exists = std::fs::exists(path);

    if exists.is_err() {
        return Err(exists.err().unwrap());
    }

    Ok(exists.ok().unwrap())
}

fn file_exists(path: &Path) -> Result<bool, Error> {
    path.try_exists()
}

pub fn check_filesystem_integrity() -> Result<bool, Error> {
    let mut did_changes = false;

    let dirs: Vec<&Path> = vec![
        Path::new("./user/"),
        Path::new("./tmp/"),

    ];


    let config_path = Path::new("./user/config.toml");

    for dir in dirs {
        let res = dir_exists(dir);

        if res.is_err() {
            return Err(Error::from(ErrorKind::PermissionDenied));
        }

        if !res.ok().unwrap() {
            std::fs::create_dir(dir)?;
            did_changes = true;
        }
    }

    if !file_exists(config_path)? {
        write_default_config(config_path);
    }


    Ok(did_changes)
}

fn write_default_config(path: &Path) -> Option<Error> {

    let conf = config::Config {
        scan_settings: ScanSettings {
            show_console: true,
            show_debug: false
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dir_doesnt_exist() {
        let path = Path::new("./nonexistantdirectory/");
        let result = dir_exists(path);

        assert!(result.is_ok());
        assert!(!result.ok().unwrap());
    }

    #[test]
    fn config_file_doesnt_exist() {
        let path = Path::new("./user/missingconfig.toml");

        let result = file_exists(path);

        assert!(result.is_ok());
        assert!(!result.ok().unwrap());
    }

    #[test]
    fn filesystem_empty() {
        let res = check_filesystem_integrity();

        assert!(res.is_ok());

        assert!(res.ok().unwrap());
    }
}