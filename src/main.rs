use std::io::{Write, Read, stdin, stdout};
use std::collections::HashMap;
use std::process::Command;

fn main() {
    let mut database = HashMap::new();
    let engineering = String::from("Engineering");
    let sale = String::from("Sale");

    database.insert(&engineering, Vec::new());
    database.insert(&sale, Vec::new());

    loop {
        println!("[1] Add someone to group");
        println!("[2] Show people from groups");
        println!("[3] Kill program");
        print!("Option: ");
        let mut buf = String::new();
        let choose = get_from_user(&mut buf);

        if choose == "1" {
            loop {
                println!("[1] To Engineering");
                println!("[2] To Sale");
                print!("Option: ");
                let mut buf = String::new();
                let choose = get_from_user(&mut buf);

                if choose != "1" && choose != "2" { continue; }

                print!("Type name: ");
                let mut buf = String::new();
                let name = get_from_user(&mut buf);

                if choose == "1" {
                    database.get_mut(&engineering).unwrap().push(String::from(name));
                    database.get_mut(&engineering).unwrap().sort();
                } else if choose == "2" {
                    database.get_mut(&sale).unwrap().push(String::from(name));
                    database.get_mut(&sale).unwrap().sort();
                }

                break;
            }
        } else if choose == "2" {
            println!("{:#?}", database);
        } else if choose == "3" {
            break;
        } else {
            continue;
        }

        clear_console();
    }
}

fn get_from_user(buf: &mut String) -> &str {
    stdout().flush().unwrap();

    stdin()
        .read_line(buf)
        .expect("Failed to read line!");

    buf.trim()
}

fn clear_console() {
    let _ = Command::new("pwsh.exe").arg("/c").arg("pause").status();

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}