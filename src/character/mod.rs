use crate::client::go_game_go;
use crate::error::{Error, Result};
use crate::player::PlayerId;
use crate::router::{Address, RouterMessage};
use crate::user::UserStream;

type Ident = String;
type Name = String;

enum CharacterCreation {
    Start,
    Name(Ident),
    NameAndIdent(Name, Ident),
}

#[derive(Debug)]
pub enum CharacterState {
    Idle,
    Combat,
    Resting,
    Dead,
}

#[derive(Debug)]
pub struct Character {
    ident: String,
    name: String,
    hp: usize,
    state: CharacterState,
}

async fn create_character(user: &mut UserStream) -> Result<Character> {
    let mut character = CharacterCreation::Start;

    loop {
        match character {
            CharacterCreation::Start => {
                character = CharacterCreation::Name(user.prompt(b"handle / alias: ").await?)
            }
            CharacterCreation::Name(ident) => {
                character = CharacterCreation::NameAndIdent(user.prompt(b"full name: ").await?, ident)
            }
            CharacterCreation::NameAndIdent(name, ident) => return Ok( Character {
                ident,
                name,
                hp: 18,
                state: CharacterState::Idle,
            })
        }
    }
}

pub async fn character_creation(mut user: UserStream) -> Result<()> {
    let character = loop {
        match create_character(&mut user).await {
            Err(Error::Validation(err)) => user.write_line(err.to_string().as_bytes()).await?,
            Err(err) => {
                log::error!("{err}");
                let _ = user.write_line(format!("ERR: {err}").as_bytes()).await;
                return Ok(());
            }
            Ok(character) => break character,
        }
    };

    go_game_go(user, character).await;
    Ok(())
}

pub async fn character_systems(rx: ()) {

}
