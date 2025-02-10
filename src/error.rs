use std::fmt::Debug;

#[derive(Debug)]
pub enum RelicWizardError {
    GameNotFound,
    GameMinimized,
    GameNotFocused,
    ConfigFileMissing
}

impl std::fmt::Display for RelicWizardError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            RelicWizardError::GameNotFound => {
                write!(f, "Could not find game window. Game might be closed.")
            },
            RelicWizardError::GameMinimized => {
                write!(f, "Game is minimized.")
            },
            RelicWizardError::GameNotFocused => {
                write!(f, "Game is not in focus.")
            },
            RelicWizardError::ConfigFileMissing => {
                write!(f, "Could not find config file.")
            }

        }
    }
}


impl std::error::Error for RelicWizardError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            RelicWizardError::GameNotFound => None,
            RelicWizardError::GameMinimized => None,
            RelicWizardError::GameNotFocused => None,
            RelicWizardError::ConfigFileMissing => None
        }
    }
}