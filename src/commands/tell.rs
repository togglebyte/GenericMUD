use super::Command;
use crate::player::PlayerId;

pub(super) fn parse(player_id: PlayerId, args: Option<&str>) -> Option<Command> {
    let tell = Tell { 
        target: player_id, message: args?.to_string() };
    Some(Command::Tell(tell))
}

#[derive(Debug)]
pub struct Tell { target: PlayerId, message: String }
