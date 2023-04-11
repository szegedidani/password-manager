use crate::{prompts, models::*};

pub fn go_to_dashboard(page: &mut CurrentPage) -> Result<(), String> {
    page.change_page(Page::Dashboard);
    prompts::dashboard_prompt(page)?;
    Ok(())
}

pub fn go_to_list_page(page: &mut CurrentPage) -> Result<(), String> {
    page.change_page(Page::ListEntries);
    prompts::list_page_prompt(page)?;
    Ok(())
}

pub fn go_to_add_page(page: &mut CurrentPage) -> Result<(), String> {
    page.change_page(Page::AddNewPassword);
    prompts::add_page_prompt(page)?;
    Ok(())
}

pub fn go_to_get_page(page: &mut CurrentPage) -> Result<(), String> {
    page.change_page(Page::GetPassword);
    prompts::get_page_prompt(page)?;
    Ok(())
}