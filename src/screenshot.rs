use xcap::image::RgbaImage;
use xcap::Window;
use crate::error::RelicWizardError;

pub fn get_game_screenshot(window_name: String) -> Result<RgbaImage, RelicWizardError> {
    let game_window = get_game_window(window_name);

    if let Ok(window) = game_window {

        if window.is_minimized() {
            return Err(RelicWizardError::GameMinimized)
        }
        
        if !window.is_focused() {
            return Err(RelicWizardError::GameNotFocused)
        }

        let game_img = window.current_monitor().capture_image().unwrap();

        Ok(game_img)
        
    } else {
        Err(game_window.err().unwrap())
    }


}

fn get_game_window(name: String) -> Result<Window, RelicWizardError> {
    let windows: Vec<Window> = Window::all().unwrap();

    for window in windows {
        if window.app_name() == name {
            return Ok(window)
        }
    }

    Err(RelicWizardError::GameNotFound)
}

#[cfg(test)]
mod tests {
    use crate::screenshot::get_game_window;

    #[test]
    fn game_window_not_found() {
        let game_window = get_game_window(String::from("random invalid window"));

        assert!(game_window.is_err());
    }
}