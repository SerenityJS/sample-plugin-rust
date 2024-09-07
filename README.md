# Introduction
The SerenityJS sample plugin for Rust provides a basic configuration for Rust plugin usage in the software. To get started, either clone this repo or create a new repository using this template. A firm understanding of Rust and TypeScript will be very beneficial for plugin development.

## Usage
SerenityJS provides an advanced but simple api in TypeScript. With the power of [NAPI-RS](https://napi.rs/), we can bring the Serenity api into Rust land. This option of plugin development is still in active development, and does not support all features that is included in the TypeScript api. Also, I an error occurs in Rust, it will more than likely crash the main server without the proper catching. To stay updated with the api, visit our [development](https://github.com/SerenityJS/serenityrs) repository.

## Building
Plugins developed in Rust will need to be compiled on their host machine, as Rust is a compile language. To do this, run the command `npm run build` in the source directory of your plugin. You will notice a `.node` binding is generated, this will be the entry to your plugin.

## Events
To hook events in Rust, you will use the name scheme of `on/before/after` plus the name of the event. For example: `before_player_chat`. This allows Serenity to apply the proper hooks for your function.

```rs
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
```