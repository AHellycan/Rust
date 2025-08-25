use crate::{models::{Epic, Story, Status}, io_utils::get_user_input};

pub struct Prompts {
    pub create_epic: Box<dyn Fn() -> Epic>,
    pub create_story: Box<dyn Fn() -> Story>,
    pub delete_epic: Box<dyn Fn() -> bool>,
    pub delete_story: Box<dyn Fn() -> bool>,
    pub update_status: Box<dyn Fn() -> Option<Status>>
}

impl Prompts {
    pub fn new() -> Self {
        Self { 
            create_epic: Box::new(create_epic_prompt),
            create_story: Box::new(create_story_prompt),
            delete_epic: Box::new(delete_epic_prompt),
            delete_story: Box::new(delete_story_prompt),
            update_status: Box::new(update_status_prompt)
        }
    }
}

fn create_epic_prompt() -> Epic {
    
    println!("----------------------------");
    println!("Epic Name: ");
    let new_epic_name = get_user_input();
    println!("test");
    println!("Epic Description: ");
    let new_epic_description = get_user_input();
    println!("test");
    let new_epic = Epic::new(new_epic_name, new_epic_description);
    new_epic
}

fn create_story_prompt() -> Story {
    println!("----------------------------");
    println!("Story Name: ");
    let new_story_name = get_user_input();
    println!("test");
    println!("Story Description: ");
    let new_story_description = get_user_input();
    println!("test");
    let new_story = Story::new(new_story_name, new_story_description);
    new_story
}

fn delete_epic_prompt() -> bool {
    println!("----------------------------");
    println!("Are you sure you want to delete this epic? All stories in this epic will also be deleted [Y/n]:");
    let new_delete_epic = get_user_input();
    match new_delete_epic.trim().to_lowercase().as_str() {
        "y" | "yes" | "" => true,
        "n" | "no" => false,
        _ => {
            println!("Invalid input. Please enter Y or n.");
            delete_epic_prompt()
        }
    }
}

fn delete_story_prompt() -> bool {
    println!("----------------------------");
    println!("Are you sure you want to delete this story? [Y/n]:");
    let new_delete_story = get_user_input();
    match new_delete_story.trim().to_lowercase().as_str() {
        "y" | "yes" | "" => true,
        "n" | "no" => false,
        _ => {
            println!("Invalid input. Please enter Y or n.");
            delete_epic_prompt()
        }
    }
}

fn update_status_prompt() -> Option<Status> {
    println!("----------------------------");
    println!("New Status (1 - OPEN, 2 - IN-PROGRESS, 3 - RESOLVED, 4 - CLOSED) :");
    let new_status = get_user_input();
    match new_status.trim() {
        "1" => Some(Status::Open),
        "2" => Some(Status::InProgress),
        "3" => Some(Status::Resolved),
        "4" => Some(Status::Closed),
        "" => None,
        _ => {
            println!("Invalid input. Please enter a number between 1 and 4.");
            update_status_prompt()
        }
    }
}