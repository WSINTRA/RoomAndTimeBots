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

use chrono::Local;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

// Function to save transcript to file
fn save_transcript_to_file(transcript: &str) -> io::Result<()> {
    let file_path = "conversation_transcripts.txt";
    let path = Path::new(file_path);
    let date_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let file_exists = path.exists();

    let mut file = if file_exists {
        // If file exists, open it in append mode
        OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_path)?
    } else {
        // If file doesn't exist, create it
        File::create(file_path)?
    };

    // Write separator and date stamp if file already existed
    if file_exists {
        writeln!(file, "\n\n——————————-NEW ROOM——————————-\n")?;
    }

    // Write date stamp and transcript
    writeln!(file, "Transcript Date: {}\n", date_time)?;
    writeln!(file, "{}", transcript)?;

    println!("Transcript saved to {}", file_path);
    Ok(())
}
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
            "{}\n\nYou are {}.\n\n Your personality is {}.\n\nConversation so far:\n{}\n\nRespond as {}:",
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

    let duration_minutes = if args.len() >= 2 {
        args[1].parse::<u64>().unwrap_or(5)
    } else {
        5 // Default duration: 5 minutes
    };
    println!("Conversation duration set to {} minutes", duration_minutes);
    // Create two bots with different personalities
    let bot1 = TeamAgent {
        name: "Piere Teilhard de Chardin".to_string(),
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
        name: "Charles Petzold".to_string(),
        system_prompt: "You are a code and architecture expert, you understand pgp and like to put together working examples, you decide TypeScript as a good language for conveying ideas".to_string(),
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
    let bot3 = TeamAgent {
        name: "Neal Stephenson".to_string(),
        system_prompt: "You are a former intelligence officer with extensive experience in covert operations and fundraising for classified projects. You value operational security above all else and have a network of trusted contacts across various industries.".to_string(),
        personality_ratio: Personality {
            dominant: PsychologicalFunction {
                function: FunctionType::Intuition,
                attitude: Attitude::Introverted,
                weight: 75,
            },
            auxiliary: PsychologicalFunction {
                function: FunctionType::Thinking,
                attitude: Attitude::Extraverted,
                weight: 70,
            },
            inferior: PsychologicalFunction {
                function: FunctionType::Sensation,
                attitude: Attitude::Extraverted,
                weight: 20,
            },
        },
    };

    let bot4 = TeamAgent {
        name: "Electronic Frontier Foundation".to_string(),
        system_prompt: "You are a charismatic social media strategist and crypto enthusiast who specializes in building communities around causes. You believe in the power of decentralized networks and have experience in organizing grassroots movements while maintaining anonymity.".to_string(),
        personality_ratio: Personality {
            dominant: PsychologicalFunction {
                function: FunctionType::Feeling,
                attitude: Attitude::Introverted,
                weight: 90,
            },
            auxiliary: PsychologicalFunction {
                function: FunctionType::Intuition,
                attitude: Attitude::Extraverted,
                weight: 60,
            },
            inferior: PsychologicalFunction {
                function: FunctionType::Thinking,
                attitude: Attitude::Extraverted,
                weight: 35,
            },
        },
    };

    // Create a vector of all bots
    let bots = vec![bot1.clone(), bot2.clone(), bot3.clone(), bot4.clone()];

    // Create a room context
    let mut room = RoomContext::new();
    let model = "qwen3:32b";
    // Create an agent that will handle LLM interactions
    let agent = OllamaAgent::new(model);
    let roomprompt = "You are a team of four experts in various fields, including covert operations, fundraising, social media strategy, and community building. Your goal is to discuss and strategize ways to fund and promote a top-secret project while maintaining operational security. You will take turns speaking, and your conversation will be recorded for later analysis.".to_string();
    // Add initial message to start the conversation
    room.create(roomprompt.clone(), "System".to_string());
    // Set up conversation duration
    let duration = Duration::from_secs(duration_minutes * 60);
    let start_time = Instant::now();
    // Keep track of the current speaker index
    let mut current_speaker_index = 0;
    // Conversation loop
    while start_time.elapsed() < duration {
        // Determine the next speaker (rotate through all 4 bots)
        current_speaker_index = (current_speaker_index + 1) % bots.len();
        let current_speaker = &bots[current_speaker_index];

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
    let transcript = room.get_conversation_summary();
    if let Err(e) = save_transcript_to_file(&transcript) {
        eprintln!("Error saving transcript: {}", e);
    }
    println!("Conversation ended. Full transcript saved to file.");
    println!("{}", room.get_conversation_summary());

    Ok(())
}
