use crate::menu::menu_implementation::{new_menu, MenuEntry};

#[allow(dead_code)]
pub fn menu_main() {
    let menu_options: Vec<MenuEntry> = vec![
       MenuEntry {
            designation: String::from("Version"),
            action: Some(get_version)
        },
        MenuEntry {
            designation: String::from("Settings"),
            action: Some(menu_settings)
        },
        MenuEntry {
            designation: String::from("Exit"),
            action: None
        }
    ];

    new_menu(String::from("Main Menu"), menu_options);
}

#[allow(dead_code)]
pub fn menu_settings() {
    let menu_options: Vec<MenuEntry> = vec![
        MenuEntry {
            designation: String::from("Back"),
            action: None
        }
    ];

    new_menu(String::from("Settings"), menu_options);
}

#[allow(dead_code)]
fn get_version() {
    println!("Relicwizard v{}", env!("CARGO_PKG_VERSION"));
}