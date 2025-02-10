use std::io;

#[allow(dead_code)]
pub struct MenuEntry {
    pub designation: String,
    pub action: Option<fn()>
}

#[allow(dead_code)]
pub fn new_menu(menu_name: String, entries: Vec<MenuEntry>) {
    loop {
        let choice: i8 = loop {
            println!("====[{menu_name}]====");
            print_entries(&entries);


            let res = get_menu_input(entries.len() as i8);

            if res.is_ok() {
                break res.ok().unwrap() - 1;
            }

            println!("error: {}", res.err().unwrap());
        };

        if entries[choice as usize].action.is_some() {
            (entries[choice as usize].action.unwrap())();
        } else {
            break;
        }
    }
}
#[allow(dead_code)]
fn print_entries(entries: &Vec<MenuEntry>) -> i8 {
    let mut i:i8 = 1;

    for entry in entries {

        println!("({i}) - {}", entry.designation);

        i += 1;
    }
    i
}

#[allow(dead_code)]
fn get_menu_input(max_input: i8) -> Result<i8, Box<dyn std::error::Error>> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let num:i8 = input.trim().parse::<i8>()?;

    if num > max_input {
        Err("input is too large")?;
    }

    if num < 1 {
        Err("input is too small")?;
    }

    Ok(num)
}