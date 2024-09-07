use napi_derive::napi;

use serenityrs::world::events::player_join::PlayerJoinSignal;

#[napi]
pub fn on_player_join(event: PlayerJoinSignal) {
  // Get the player from the event
  let player = event.player;

  // Send the player a welcome message
  player.send_message("Welcome to the server!");
}
