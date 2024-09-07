use napi_derive::napi;

use serenityrs::serenity::serenity::Serenity;
use serenityrs::plugin::Plugin;

/**
 * Fires when the plugin is initialized
*/
#[napi]
pub fn on_initialize(_serenity: Serenity, plugin: Plugin) {
  // Get the logger of the plugin
  let logger = plugin.logger;

  // Log that the plugin has been initialized
  logger.info("Plugin initialized!");
}

/**
 * Fire when the plugin is started
*/
#[napi]
pub fn on_startup(_serenity: Serenity, plugin: Plugin) {
  // Get the logger of the plugin
  let logger = plugin.logger;

  // Log that the plugin has been started
  logger.info("Plugin started!");
}

/**
 * Fire when the plugin is stopped
*/
#[napi]
pub fn on_shutdown(_serenity: Serenity, plugin: Plugin) {
  // Get the logger of the plugin
  let logger = plugin.logger;

  // Log that the plugin has been stopped
  logger.info("Plugin stopped!");
}

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