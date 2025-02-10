use std::io::{stderr, Write};

mod menu;
mod screenshot;
mod error;
mod init;
mod config;

fn main() {
    // menus::menu_main()
    let res = init::check_filesystem_integrity();
    
    if res.is_err() {
        println!("{:?}", res.err().unwrap());
    }
}
