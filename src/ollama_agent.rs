use crate::bot_agent::BotAgent;
use crate::jungian_modelling::{Attitude, FunctionType, Personality};
use crate::room_setup::{RoomContext, TeamAgent};
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;

pub struct OllamaAgent {
    client: Ollama,
    model: String,
}

impl OllamaAgent {
    pub fn new(model: &str) -> Self {
        OllamaAgent {
            client: Ollama::default(),
            model: model.to_string(),
        }
    }
}

impl BotAgent for OllamaAgent {
    async fn generate_response(&self, room_context: &RoomContext, agent: &TeamAgent) -> String {
        let conversation = room_context.get_conversation_summary();
        let prompt = format!(
            "System Prompt: {}\n\n Name: {}.\n\n Personality type: {}.\n\nConversation so far:\n{}:",
            agent.system_prompt,
            agent.name,
            self.personality_description(&agent.personality_ratio),
            conversation,
        );
        println!(
            "Generating response for {} with prompt: {}",
            agent.name,
            self.personality_description(&agent.personality_ratio)
        );
        let request = GenerationRequest::new(self.model.clone(), prompt);
        match self.client.generate(request).await {
            Ok(response) => response.response,
            Err(e) => {
                eprintln!("Error generating response: {}", e);
                format!("I'm having trouble responding right now.")
            }
        }
    }

    //    fn invalidate_messages(&self, _room_context: &mut RoomContext, _agent: &TeamAgent) {
    //        // Not implemented for this prototype
    //    }
}

impl OllamaAgent {
    fn personality_description(&self, personality: &Personality) -> String {
        format!(
            "{} {} ({}%), {} {} ({}%), with inferior {} {} ({}%)",
            self.attitude_str(personality.dominant.attitude),
            self.function_str(personality.dominant.function),
            personality.dominant.weight,
            self.attitude_str(personality.auxiliary.attitude),
            self.function_str(personality.auxiliary.function),
            personality.auxiliary.weight,
            self.attitude_str(personality.inferior.attitude),
            self.function_str(personality.inferior.function),
            personality.inferior.weight
        )
    }

    fn attitude_str(&self, attitude: Attitude) -> &'static str {
        match attitude {
            Attitude::Introverted => "Introverted",
            Attitude::Extraverted => "Extraverted",
        }
    }

    fn function_str(&self, function: FunctionType) -> &'static str {
        match function {
            FunctionType::Thinking => "Thinking",
            FunctionType::Feeling => "Feeling",
            FunctionType::Sensation => "Sensation",
            FunctionType::Intuition => "Intuition",
        }
    }
}
