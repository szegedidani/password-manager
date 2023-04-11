use std::{io, fs};

use magic_crypt::{new_magic_crypt, MagicCrypt256, MagicCryptTrait};

use crate::{models::*, navigation::*, copy};

pub fn dashboard_prompt(page: &mut CurrentPage) -> Result<(), String> {
    println!("Select action");
    println!("1 - List titles, 2 - Get password, 3 - Add new password, 4 - Exit");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input");
        
    return match input.trim() {
        "1" => go_to_list_page(page),
        "2" => go_to_get_page(page),
        "3" => go_to_add_page(page),
        "4" => Ok(()),
        _ => return Err("Selected page doesn't exist".to_owned()),
    }
}

pub fn list_page_prompt(page: &mut CurrentPage) -> Result<(), String> {
    println!("Select action");
    println!("1 - Back to Dashboard, 2 - Exit");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input");
        
    return match input.trim() {
        "1" => go_to_dashboard(page),
        "2" => Ok(()),
        _ => return Err("Selected page doesn't exist".to_owned()),
    }
}

pub fn get_page_prompt(page: &mut CurrentPage) -> Result<(), String> {
    println!("Select action");
    println!("1 - Get password, 2 - Back to Dashboard, 3 - Exit");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input");
        
    return match input.trim() {
        "1" => get_password_prompt(page),
        "2" => go_to_dashboard(page),
        "3" => Ok(()),
        _ => return Err("Selected page doesn't exist".to_owned()),
    }
}

pub fn add_page_prompt(page: &mut CurrentPage) -> Result<(), String> {
    println!("Select action");
    println!("1 - Add new password, 2 - Back to Dashboard, 3 - Exit");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input");
        
    return match input.trim() {
        "1" => add_password_prompt(page),
        "2" => go_to_dashboard(page),
        "3" => Ok(()),
        _ => return Err("Selected page doesn't exist".to_owned()),
    }
}

pub fn get_password_prompt(page: &mut CurrentPage) -> Result<(), String> {
    println!("Enter title");
    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Invalid input");

    let mc: MagicCrypt256 = new_magic_crypt!("magickey", 256);

    let data = fs::read_to_string("./data/datas.json")
        .expect("Unable to read file");
    let json: JSONData = serde_json::from_str(&data)
        .expect("JSON does not have correct format.");

    let selected_pass = json.passwords.iter().find(|password| password.title == title.trim()).or_else(|| {
        io::stdin().read_line(&mut title).expect("Invalid input");
        None
    }).ok_or(format!("password not found"))?;

    let decrypted = mc.decrypt_base64_to_string(selected_pass.password.clone()).expect("decryption failed");
    copy(&decrypted);

    get_page_prompt(page)
}

pub fn add_password_prompt(page: &mut CurrentPage) -> Result<(), String> {
    let mut title = String::new();
    let mut password = String::new();

    println!("Enter title");
    io::stdin().read_line(&mut title).expect("Invalid input");

    println!("Enter password");
    io::stdin().read_line(&mut password).expect("Invalid input");

    let mc: MagicCrypt256 = new_magic_crypt!("magickey", 256);
    let record = Password::new(title.trim().to_owned(), password.trim().to_owned(), mc);

    let data = fs::read_to_string("./data/datas.json")
        .expect("Unable to read file");
    let mut json: JSONData = serde_json::from_str(&data)
        .expect("JSON does not have correct format.");

    json.passwords.push(record);

    let contents = serde_json::to_string(&json).map_err(|err| format!("{err:?}"))?;
    fs::write("./data/datas.json", contents).map_err(|err| format!("{err:?}"))?;

    add_page_prompt(page)
}