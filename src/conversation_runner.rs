use crate::bot_agent::BotAgent;
use crate::ollama_agent::OllamaAgent;
use crate::room_setup::{RoomContext, TeamAgent};
use rand::prelude::*;
use std::error::Error;
use std::time::{Duration, Instant};
use tokio::time::sleep;

pub async fn run_conversation(
    start_time: Instant,
    duration: Duration,
    bots: &Vec<TeamAgent>,
    room: &mut RoomContext,
    agent: &OllamaAgent,
    message_delay: Duration,
) -> Result<(), Box<dyn Error>> {
    let mut rng = rand::rng();
    let mut current_speaker_index = rng.random_range(0..bots.len());
    while start_time.elapsed() < duration {
        // Determine the next speaker (rotate through all bots)
        current_speaker_index = (current_speaker_index + 1) % bots.len();
        let current_speaker = &bots[current_speaker_index];

        println!("{} ruminating...", current_speaker.name);

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
pub async fn end_conversation(
    bots: &Vec<TeamAgent>,
    room: &mut RoomContext,
    agent: &OllamaAgent,
) -> Result<(), Box<dyn Error>> {
    // Generate final response
    let final_response = agent.generate_response(&room, &bots[4]).await;

    // Add final response to the room
    room.create(final_response.clone(), "System".to_string());

    // Print final response
    println!("Final thoughts: {}", final_response);

    Ok(())
}
