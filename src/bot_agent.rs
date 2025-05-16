// bot_agent.rs
use crate::room_setup::{RoomContext, TeamAgent};

pub trait BotAgent {
    async fn generate_response(&self, room_context: &RoomContext, agent: &TeamAgent) -> String;
    fn invalidate_messages(&self, room_context: &mut RoomContext, agent: &TeamAgent);
}
