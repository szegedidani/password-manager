// Pages: 
//      Master Password page ->
//          prompts: 
//              enter master password
//          errors:
//              invalid input
//              wrong password
//      Dashboard page ->
//          propmts:
//              select page
//          errors:
//              invalid select
//      List entries page ->
//          prompts:
//              select action
//          errors:
//              invalid action
//      Add new password page ->
//          prompts:
//              select action
//              enter title
//              enter password
//          errors:
//              invalid action
//              invalid title
//              invalid password
//      Get password page ->
//          prompts:
//              select action
//              enter title
//          errors:
//              invalid action
//              invalid title
//              password not found

pub mod models;
pub mod prompts;
pub mod navigation;

use cli_clipboard;
use std::fs;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::{error::Error, io, fmt::format, rc::Rc};

use magic_crypt::{new_magic_crypt, MagicCryptTrait, MagicCrypt256};
use models::*;

fn main() {
    flow();
}

fn flow() {
    let app_state = run_app();

    match app_state {
        Ok(_) => {
            println!("Shutting down");
        },
        Err(err) => {
            println!("{err:?}");
            println!("Press Enter to continue!");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Invalid input");
            flow();
        }
    }
}
    // let mc: MagicCrypt256 = new_magic_crypt!("magickey", 256);
    // let passwords: Vec<Password> = vec![Password::new("facebook".to_owned(), "asdqwe123".to_owned(), mc.clone())];
    
    // let mut input = get_user_input(UserInputType::EnterTitle)?;

    // let selected_pass = passwords.iter().find(|password| password.title == input.trim()).or_else(|| {
    //     io::stdin().read_line(&mut input).expect("Invalid input");
    //     None
    // }).ok_or(format!("password not found"))?;

    // let decrypted = mc.decrypt_base64_to_string(selected_pass.password.clone()).expect("decryption failed");
    // copy(&decrypted);

    // Ok(())

fn run_app() -> Result<(), Box<dyn Error>> {
    // let contents: JSONData = JSONData { 
    //     master_password: "asd".to_owned(),
    //     passwords: vec![]
    // };
    // let contents = serde_json::to_string(&contents)?;

    // fs::write("./data/datas.json", contents)?;

    let mut page = CurrentPage::new();

    page.display_page();
    
    println!("Enter master password!\n");
    let input = rpassword::read_password().unwrap();
    authenticate(input.trim())?;

    navigation::go_to_dashboard(&mut page)?;
    
    Ok(())
}

fn authenticate(credentials: &str) -> Result<(), String> {
    let mut s = DefaultHasher::new();
    credentials.to_owned().hash(&mut s);
    let hash = s.finish();

    let data = fs::read_to_string("./data/datas.json")
        .expect("Unable to read file");
    let json: JSONData = serde_json::from_str(&data)
        .expect("JSON does not have correct format.");

    let str = format!("{}", hash);

    if json.master_password == str {
        return Ok(());
    } else {
        return Err(format!("Wrong password"));
    }
}

fn copy(the_string: &str) {
    cli_clipboard::set_contents(the_string.to_owned()).unwrap();
    assert_eq!(cli_clipboard::get_contents().unwrap(), the_string);
}