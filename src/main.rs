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
use std::{error::Error, io, rc::Rc};

use magic_crypt::{new_magic_crypt, MagicCrypt256, MagicCryptTrait};
use models::*;

fn main() {
    let mc: Rc<MagicCrypt256> = Rc::new(new_magic_crypt!("magickey", 256));

    let data = fs::read_to_string("./data/datas.json").expect("Unable to read file");
    let json: EncryptedJSONData = serde_json::from_str(&data).expect("JSON does not have correct format.");
    let decrypted_string = mc.decrypt_base64_to_string(json.encrypted_data).expect("decrypt");
    let decrypted_json_data: DecryptedJSONData = serde_json::from_str(&decrypted_string).expect("JSON does not have correct format.");

    println!("{:?}", decrypted_json_data.encrypted_data.passwords[0].title);
    

    let decrypted_string = serde_json::to_string(&decrypted_json_data).map_err(|err| format!("{err:?}")).expect("tostring");
    let enccrypted_string = mc.encrypt_str_to_base64(decrypted_string);
    let encrypted_data = EncryptedJSONData {
        encrypted_data: enccrypted_string
    };
    let encrypted_json = serde_json::to_string(&encrypted_data).map_err(|err| format!("{err:?}")).expect("tostring");
    fs::write("./data/datas.json", &encrypted_json).map_err(|err| format!("{err:?}")).expect("write");
    // flow(mc.clone());
}

fn flow(mc: Rc<MagicCrypt256>) {
    let app_state = run_app(mc.clone());
    match app_state {
        Ok(_) => {
            println!("Shutting down");
        },
        Err(err) => {
            println!("{err:?}");
            println!("Press Enter to continue!");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Invalid input");
            flow(mc);
        }
    }
}

fn run_app(mc: Rc<MagicCrypt256>) -> Result<(), Box<dyn Error>> {

    let mut page = CurrentPage::new(mc);

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