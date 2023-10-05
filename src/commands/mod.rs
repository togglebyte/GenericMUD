use crate::player::PlayerId;
use crate::room::RoomId;
use self::tell::Tell;

mod tell;

#[derive(Debug)]
pub enum Command {
    Tell(Tell),
    Say(RoomId, String),
}

fn parse_command(player_id: PlayerId, ident: &str, args: Option<&str>) -> Option<Command> {
    match ident {
        "tell" | "t" => tell::parse(player_id, args),
        _ => None,
    }
}

pub fn parse(player_id: PlayerId, input: &str) -> Option<Command> {
    let input = input.trim();

    // input = tell bob hi

    match input.split_once(char::is_whitespace) {
        Some((cmd, args)) => parse_command(player_id, cmd, Some(args.trim())),
        None => parse_command(player_id, input, None)
    };

    panic!()
}
