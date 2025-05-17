// room_setup.rs
use crate::jungian_modelling::Personality;
#[derive(Debug, Clone)] // Added Clone trait here
pub struct TeamAgent {
    pub name: String,
    pub system_prompt: String,
    pub personality_ratio: Personality,
}

// Bot struct that uses the TeamMember enum for roles
//#[derive(Debug)]
//struct Bot {
//    id: String,
//    role: TeamAgent,
//}

// Message struct to hold each conversation message
#[derive(Debug, Clone)]
pub struct Message {
    pub content: String,
    pub author: String,
    pub is_valid: bool,
}

// RoomContext struct to manage the conversation history
#[derive(Debug)]
pub struct RoomContext {
    messages: Vec<Message>,
}

impl RoomContext {
    pub fn new() -> Self {
        RoomContext {
            messages: Vec::new(),
        }
    }

    // Add a new message to the conversation
    pub fn create(&mut self, content: String, author: String) {
        self.messages.push(Message {
            content,
            author,
            is_valid: true,
        });
    }

    // Get all valid messages
    pub fn read(&self) -> Vec<&Message> {
        self.messages.iter().filter(|m| m.is_valid).collect()
    }

    // Delete (invalidate) messages that match a keyword
    // pub fn delete_by_keyword(&mut self, keyword: &str) {
    //     for msg in &mut self.messages {
    //         if msg.content.contains(keyword) {
    //             msg.is_valid = false;
    //         }
    //     }
    // }

    // Get conversation summary as a formatted string
    pub fn get_conversation_summary(&self) -> String {
        self.read()
            .iter()
            .map(|msg| format!("{}: {}", msg.author, msg.content))
            .collect::<Vec<String>>()
            .join("\n")
    }
}
