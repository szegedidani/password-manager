use std::rc::Rc;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use magic_crypt::{MagicCrypt256, MagicCryptTrait};

pub enum Page {
    Login,
    Dashboard,
    ListEntries,
    AddNewPassword,
    GetPassword,
}

#[derive(Serialize, Deserialize)]
pub struct Password {
    pub title: String,
    pub password: String,
}

impl Password {
    pub fn new(title: String, password: String, mc: Rc<MagicCrypt256>) -> Self {
        let password = mc.encrypt_str_to_base64(password);
        Self { title, password }
    }
}
enum UserInputType {
    EnterTitle,
    EnterMasterPassword,
    // EnterNewPassword
}

pub struct CurrentPage {
    pub page: Page,
    pub mc: Rc<MagicCrypt256>,
}

#[derive(Serialize, Deserialize)]
pub struct EncryptedJSONData {
    pub encrypted_data: String,
}
#[derive(Serialize, Deserialize)]
pub struct DecryptedJSONData {
    pub encrypted_data: JSONData,
}
#[derive(Serialize, Deserialize)]
pub struct JSONData {
    pub master_password: String,
    pub passwords: Vec<Password>,
}

impl CurrentPage {
    pub fn new(mc: Rc<MagicCrypt256>) -> Self {
        let page = CurrentPage { page: Page::Login, mc };
        page
    }

    pub fn change_page(&mut self, new_page: Page) {
        self.page = new_page;
        self.display_page();
    }

    pub fn display_page(&self) {
        print!("\x1B[2J\x1B[1;1H");
        
        match self.page {
            Page::Login => println!("<------------Login------------>"),
            Page::Dashboard => println!("<------------Dashboard------------>"),
            Page::ListEntries => println!("<------------ListEntries------------>"),
            Page::AddNewPassword => println!("<------------AddNewPassword------------>"),
            Page::GetPassword => println!("<------------GetPassword------------>")
        }
    }
}