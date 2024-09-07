use napi_derive::napi;
use napi::Result;

use serenityrs::world::events::player_chat::PlayerChatSignal;
use serenityrs::world::components::player::component::PlayerComponent;

#[napi]
pub fn before_player_chat(event: PlayerChatSignal) -> Result<bool> {
  // Get the player from the event
  let player = event.player;

  // Get the players health component
  let health = PlayerComponent::get_health(&player);

  // Get the current and max health of the player
  let current_health = health.get_current_value();
  let max_health = health.effective_max;

  // Send the player their current health
  player.send_message(format!("Your current health is {}/{}", current_health, max_health).as_str());

  // Cancel the event
  Ok(false)
}
