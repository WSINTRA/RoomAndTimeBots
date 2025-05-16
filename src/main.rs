// main.rs
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;
use std::env;
use std::error::Error;
use std::time::{Duration, Instant};
use tokio::time::sleep;

mod bot_agent;
mod jungian_modelling;
mod room_setup;

use bot_agent::BotAgent;
use jungian_modelling::{Attitude, FunctionType, Personality, PsychologicalFunction};
use room_setup::{RoomContext, TeamAgent};

struct OllamaAgent {
    client: Ollama,
    model: String,
}

impl OllamaAgent {
    fn new(model: &str) -> Self {
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
            "{}\n\nYou are {}. Your personality is {}.\n\nConversation so far:\n{}\n\nRespond as {}:",
            agent.system_prompt,
            agent.name,
            self.personality_description(&agent.personality_ratio),
            conversation,
            agent.name
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

    fn invalidate_messages(&self, _room_context: &mut RoomContext, _agent: &TeamAgent) {
        // Not implemented for this prototype
    }
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    println!("Program started with arguments: {:?}", args);

    let duration_minutes = if args.len() >= 3 {
        args[1].parse::<u64>().unwrap_or(5)
    } else {
        5 // Default duration: 5 minutes
    };
    println!("Conversation duration set to {} minutes", duration_minutes);
    // Create two bots with different personalities
    let bot1 = TeamAgent {
        name: "Alice".to_string(),
        system_prompt: "You are a noble warrior of the light, you have been living on earth for thousands of years. Your existence has been highly secretive and you are known only to the true seekers of knowledge as an Ascended Master".to_string(),
        personality_ratio: Personality {
            dominant: PsychologicalFunction {
                function: FunctionType::Thinking,
                attitude: Attitude::Introverted,
                weight: 80,
            },
            auxiliary: PsychologicalFunction {
                function: FunctionType::Intuition,
                attitude: Attitude::Extraverted,
                weight: 60,
            },
            inferior: PsychologicalFunction {
                function: FunctionType::Feeling,
                attitude: Attitude::Extraverted,
                weight: 30,
            },
        },
    };

    let bot2 = TeamAgent {
        name: "Bob".to_string(),
        system_prompt: "You are an overweight citizen of the united states, you work as a delivery driver, you have no kids and no girldfriend, you beleive in the fourth but are not actually sure you are smart enough to have conversations about politics due to a failed education system, however you are extremly well read and you do actually make valid points, you enjoy anarchist literature and struggle with inferiority complexes".to_string(),
        personality_ratio: Personality {
            dominant: PsychologicalFunction {
                function: FunctionType::Feeling,
                attitude: Attitude::Extraverted,
                weight: 85,
            },
            auxiliary: PsychologicalFunction {
                function: FunctionType::Sensation,
                attitude: Attitude::Introverted,
                weight: 65,
            },
            inferior: PsychologicalFunction {
                function: FunctionType::Thinking,
                attitude: Attitude::Introverted,
                weight: 25,
            },
        },
    };

    // Create a room context
    let mut room = RoomContext::new();

    // Create an agent that will handle LLM interactions
    let agent = OllamaAgent::new("hf.co/mlabonne/gemma-3-27b-it-abliterated-GGUF:Q8_0");

    // Add initial message to start the conversation
    room.create(
        "Codes secrecy and societal change from within".to_string(),
        bot1.name.clone(),
    );

    // Set up conversation duration
    let duration = Duration::from_secs(duration_minutes * 60);
    let start_time = Instant::now();

    // Conversation loop
    while start_time.elapsed() < duration {
        // Get the last speaker
        let messages = room.read();
        let last_message = messages.last().unwrap();
        let current_speaker = if last_message.author == bot1.name {
            &bot2
        } else {
            &bot1
        };

        println!("\n{} is thinking...", current_speaker.name);

        // Generate response
        let response = agent.generate_response(&room, current_speaker).await;

        // Add response to the room
        room.create(response.clone(), current_speaker.name.clone());

        println!("{}: {}", current_speaker.name, response);

        // Wait a bit before the next message
        sleep(Duration::from_secs(3)).await;

        // Check if time is up
        if start_time.elapsed() >= duration {
            break;
        }
    }
    println!("\nConversation ended. Full transcript:");
    println!("{}", room.get_conversation_summary());

    Ok(())
}
