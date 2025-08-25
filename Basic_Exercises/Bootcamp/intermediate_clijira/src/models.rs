use serde::{Serialize, Deserialize};
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    NavigateToEpicDetail { epic_id: u32 },
    NavigateToStoryDetail { epic_id: u32, story_id: u32 },
    NavigateToPreviousPage,
    CreateEpic,
    UpdateEpicStatus { epic_id: u32 },
    DeleteEpic { epic_id: u32 },
    CreateStory { epic_id: u32 },
    UpdateStoryStatus { story_id: u32 },
    DeleteStory { epic_id: u32, story_id: u32 },
    Exit,
}


#[derive(Serialize, Deserialize, Debug,Clone, PartialEq, Eq)]
pub enum Status {
    // TODO: add fields (make sure the fields are public)
    Open,
    InProgress,
    Resolved,
    Closed,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open => write!(f, "OPEN"),
            Self::InProgress => write!(f, "IN PROGRESS"),
            Self::Resolved => write!(f, "RESOLVED"),
            Self::Closed => write!(f, "CLOSED"),
            
        }
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct Epic {
    // TODO: add fields (make sure the fields are public) (DONE)
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u32>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Self { 
            name, 
            description, 
            status: Status::Open,
            stories: vec![] }
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct Story {
    // TODO: add fields (make sure the fields are public) (DONE)
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Self { name,
             description,
              status: Status::Open }
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct DBState {
    // This struct represents the entire db state which includes the last_item_id, epics, and stories
    // TODO: add fields (make sure the fields are public)
    pub last_item_id: u32,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>,
}
                       