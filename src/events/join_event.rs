pub use crate::send_cmd;
use crate::Player;

pub fn fire_event(player: Player) {
    // TODO: Event handler shenenegans
    send_cmd!("say Hello {} {}", player.name,player.uuid);
}