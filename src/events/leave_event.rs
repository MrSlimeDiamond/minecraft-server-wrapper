use crate::send_cmd;
use crate::Player;

pub fn fire_event(player: Player, reason: &str) {
    // TODO: Event handler shenenegans
    send_cmd!("say {} disconnected: {}",player.name,reason);
}