# Minecraft Server Wrapper
> This is **not** a replacement for a server mod. Its intension is to track and parse console output, and send commands to a server without the need for a plugin.

A wrapper for Minecraft: Java edition multiplayer servers

Tested only on Vanilla, but Bukkit/Sponge/Canary/...etc will likely work with minimal modification.

Fires events, sends commands to the server via a [JSON API](https://github.com/MrSlimeDiamond/minecraft-server-wrapper/wiki/JSON-API)

## Usage
### Requirements
* Rust Nightly
### Setting up
> This might change
1. Clone this repository into your Minecraft servers root directory (i.e: /home/minecraft/minecraft_server)
2. Use `cargo run` to run the configured file under the wrapper.
