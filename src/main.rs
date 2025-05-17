// main.rs
use std::env;
use std::error::Error;
use std::io::stdout;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::task;
use tokio::time::sleep;

mod bot_agent;
mod bot_config;
mod conversation_runner;
mod jungian_modelling;
mod ollama_agent;
mod progress_bar;
mod room_setup;
mod transcript;

use bot_agent::BotAgent;
use bot_config::create_bots;
use conversation_runner::run_conversation;
use ollama_agent::OllamaAgent;
use progress_bar::ProgressBar;
use room_setup::RoomContext;

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

    // Create a channel to signal the progress bar task to stop
    let (stop_tx, mut stop_rx) = mpsc::channel::<bool>(1);

    // Create bots from configuration
    let bots = create_bots();

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

    // Start a separate task for updating the progress bar
    let progress_task = task::spawn(async move {
        let mut progress_bar = ProgressBar::new(stdout(), 50);
        progress_bar.init().unwrap();
        loop {
            // Check if we should stop
            if let Ok(_) = stop_rx.try_recv() {
                break;
            }
            // Update progress bar
            let elapsed = start_time.elapsed();
            if elapsed >= duration {
                break;
            }
            progress_bar.update(elapsed, duration).unwrap();
            // Sleep for 1 second
            sleep(Duration::from_secs(1)).await;
        }
        progress_bar.cleanup().unwrap();
        // Return the progress bar so it can be used again if needed
        progress_bar
    });

    // Run the conversation with the improved implementation
    run_conversation(
        start_time,
        duration,
        &bots,
        &mut room,
        &agent,
        Duration::from_secs(3),
    )
    .await?;

    // Signal the progress bar task to stop
    let _ = stop_tx.send(true).await;

    // Wait for the progress bar task to finish
    let _ = progress_task.await;

    let transcript = room.get_conversation_summary();
    if let Err(e) = transcript::save_transcript_to_file(&transcript) {
        eprintln!("Error saving transcript: {}", e);
    }

    println!("Conversation ended. Full transcript saved to file.");
    println!("{}", room.get_conversation_summary());

    Ok(())
}
