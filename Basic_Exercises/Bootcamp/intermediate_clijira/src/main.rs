use core::{option::Option::Some, result::Result::Err};
use std::rc::Rc;
mod models;
mod db;
use db::*;
mod ui;
mod io_utils;
use io_utils::*;
mod navigator;
use navigator::*;


fn main() {
    // TODO: create database and navigator
    
    let db = Rc::new(JiraDatabase::new("./data/db.json".to_owned()));
    let mut navigator = Navigator::new(Rc::clone(&db));

    loop {
        clearscreen::clear().unwrap();

        // TODO: implement the following functionality:
        // 1. get current page from navigator. If there is no current page exit the loop.
        // 2. render page
        if let Some(current_page) = navigator.get_current_page() {
            if let Err(error) = current_page.draw_page() {
                println!("Error rendering page: {}\nPress any key to continue...",error);
                wait_for_key_press();
            };
           // 3. get user input
           let user_input = get_user_input();
           println!("User input: {}", user_input);
            // 4. pass input to page's input handler
            // 5. if the page's input handler returns an action let the navigator process the action
           match current_page.handle_input(user_input.trim()) {
               Ok(action) => {
                   if let Some(action) = action {
                        println!("Action: {:?}", action);
                       if let Err(e) = navigator.handle_action(action) {
                           println!("Error handling action: {}\nPress any key to continue...", e);
                           wait_for_key_press();
                       }
                   }
               }
               Err(e) => {
                   println!("Error handling input: {}\nPress any key to continue...", e);
                   wait_for_key_press();
               }
               
           } 
        } else {
            break;
        }
        
        
        
    }
}