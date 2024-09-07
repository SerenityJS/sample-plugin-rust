use napi_derive::napi;

use serenityrs::world::events::player_leave::PlayerLeaveSignal;

#[napi]
pub fn on_player_leave(event: PlayerLeaveSignal) {
  // Get the player & world from the event
  let player = event.player;
  let world = event.world;

  // Log the player that left
  world.logger.info(format!("{} has left the server!", player.username).as_str());
}
