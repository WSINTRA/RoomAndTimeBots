use crate::bot_agent::BotAgent;
use crate::ollama_agent::OllamaAgent;
use crate::room_setup::{RoomContext, TeamAgent};
use std::error::Error;
use std::time::{Duration, Instant};
use tokio::time::sleep;
/// Runs a conversation between bots for the specified duration
///
/// @param start_time The time when the conversation started
/// @param duration The maximum duration for the conversation
/// @param bots The list of bots participating in the conversation
/// @param room The conversation room
/// @param agent The agent that generates responses
/// @param message_delay The delay between messages
pub async fn run_conversation(
    start_time: Instant,
    duration: Duration,
    bots: &Vec<TeamAgent>,
    room: &mut RoomContext,
    agent: &OllamaAgent,
    message_delay: Duration,
) -> Result<(), Box<dyn Error>> {
    let mut current_speaker_index = 0;

    while start_time.elapsed() < duration {
        // Determine the next speaker (rotate through all bots)
        current_speaker_index = (current_speaker_index + 1) % bots.len();
        let current_speaker = &bots[current_speaker_index];

        println!("{} is speaking...", current_speaker.name);

        // Generate response
        let response = agent.generate_response(&room, current_speaker).await;

        // Add response to the room
        room.create(response.clone(), current_speaker.name.clone());

        // Print response
        println!("{}: {}", current_speaker.name, response);

        // Wait before the next message, but don't exceed total duration
        let remaining_time = duration.checked_sub(start_time.elapsed());
        if let Some(time_left) = remaining_time {
            if time_left < message_delay {
                // If less time remains than the delay, sleep for the remaining time
                sleep(time_left).await;
                break; // Exit after sleeping for the remaining time
            } else {
                // Otherwise, sleep for the standard delay
                sleep(message_delay).await;
            }
        } else {
            // Time is already up
            break;
        }
    }

    Ok(())
}
