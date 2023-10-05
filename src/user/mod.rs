use std::io;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufStream};
use tokio::net::TcpStream;

use crate::mpsc::{unbounded, Sender};
use crate::player::PlayerId;
use crate::router::{Address, Message};

static GREETING: &[u8] = b"Welcome to the land of GenericMUD!\n";

mod validation;

pub enum UserState {
    Anon,
    Authenticating(String),
}

struct UserStream {
    stream: BufStream<TcpStream>,
    buffer: String,
}

impl UserStream {
    async fn read(&mut self) -> &str {
        self.buffer.clear();
        self.stream.read_line(&mut self.buffer).await;
        let n = self.buffer.trim_end().len();
        self.buffer.truncate(n);
        &self.buffer
    }

    async fn write_all(&mut self, payload: &[u8]) -> io::Result<()> {
        self.stream.write_all(payload).await?;
        self.stream.flush().await;
        Ok(())
    }
}

pub async fn handle_connection(stream: TcpStream, router_tx: Sender) {
    let mut stream = UserStream {
        stream: BufStream::new(stream),
        buffer: String::new(),
    };

    let mut state = UserState::Anon;

    stream.write_all(GREETING).await;

    loop {
        // -----------------------------------------------------------------------------
        //   - Username -
        // -----------------------------------------------------------------------------
        stream.write_all(b"username: ").await;
        let username = stream.read().await;

        if let Err(e) = validate_username(&username) {
            stream.write_all(e.as_bytes()).await;
            continue;
        }
        state = UserState::Authenticating(username.into());

        // -----------------------------------------------------------------------------
        //   - Password -
        // -----------------------------------------------------------------------------
        stream.write_all(b"password: ").await;
        let password = stream.read().await;

        if let Err(e) = validate_password(&password) {
            stream.write_all(e.as_bytes()).await;
            continue;
        }

        break;
    }

    let player_id = PlayerId::next();
    let (tx, rx) = unbounded();

    // Register the new player with the router
    router_tx
        .send_async(Message::Register(tx, Address::Player(player_id)))
        .await;

    panic!("user is now authenticated");
}

fn validate_username(username: &str) -> Result<(), String> {
    validation::username_min_len(username)?;
    validation::username_max_len(username)?;
    validation::no_starting_space(username)
}

fn validate_password(password: &str) -> Result<(), String> {
    validation::password_len(password)
}
