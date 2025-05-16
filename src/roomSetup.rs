use std::collections::HashMap;
use std::env;
use std::time::Duration;

// Define the TeamMember enum with PascalCase (Rust convention)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TeamMember {
    ExpertCoder,
    Futurist,
    JungianExpert,
    Mozart,
}

// Bot struct that uses the TeamMember enum for roles
#[derive(Debug, Clone)]
struct Bot {
    id: String,
    role: TeamMember,
}

// Message struct to hold each conversation message
#[derive(Debug, Clone)]
struct Message {
    content: String,
    author: String,
    is_valid: bool,
}

// RoomContext struct to manage the conversation history
#[derive(Debug)]
struct RoomContext {
    messages: Vec<Message>,
}

impl RoomContext {
    fn new() -> Self {
        RoomContext {
            messages: Vec::new(),
        }
    }

    // Add a new message to the conversation
    fn create(&mut self, content: String, author: String) {
        self.messages.push(Message {
            content,
            author,
            is_valid: true,
        });
    }

    // Get all valid messages
    fn read(&self) -> Vec<&Message> {
        self.messages.iter().filter(|m| m.is_valid).collect()
    }

    // Delete (invalidate) messages that match a keyword
    fn delete_by_keyword(&mut self, keyword: &str) {
        for msg in &mut self.messages {
            if msg.content.contains(keyword) {
                msg.is_valid = false;
            }
        }
    }

    // Get conversation summary as a formatted string
    fn get_conversation_summary(&self) -> String {
        self.read()
            .iter()
            .map(|msg| format!("{}: {}", msg.author, msg.content))
            .collect::<Vec<String>>()
            .join("\n")
    }
}
