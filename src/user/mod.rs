use std::io;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufStream};
use tokio::net::TcpStream;

use crate::error::{Error, Result};
use crate::mpsc::RouterSender;
use crate::player::PlayerId;
use crate::router::{Address, RouterMessage};
use crate::validation;

static GREETING: &[u8] = b"Welcome to the land of GenericMUD!";

pub enum UserState {
    Anon,
    Authenticating(String),
    Authenticated,
}

pub struct UserStream {
    pub stream: BufStream<TcpStream>,
    buffer: String,
    pub router: RouterSender,
    pub state: UserState,
}

impl UserStream {
    async fn read(&mut self) -> Result<String> {
        self.buffer.clear();
        self.stream.read_line(&mut self.buffer).await?;
        let n = self.buffer.trim_end().len();
        self.buffer.truncate(n);
        let mut string = String::new();
        std::mem::swap(&mut string, &mut self.buffer);
        Ok(string)
    }

    pub async fn prompt(&mut self, prompt: &[u8]) -> Result<String> {
        self.write_all(prompt).await?;
        self.read().await
    }

    async fn read_int(&mut self) -> Result<i32> {
        panic!()
        // let s = self.read();
        // // s.parse::<
    }

    pub async fn write_line(&mut self, payload: &[u8]) -> io::Result<()> {
        self.stream.write_all(payload).await?;
        self.stream.write(b"\n").await?;
        self.stream.flush().await;
        Ok(())
    }

    async fn write_all(&mut self, payload: &[u8]) -> io::Result<()> {
        self.stream.write_all(payload).await?;
        self.stream.flush().await;
        Ok(())
    }
}

async fn authenticate(stream: &mut UserStream) -> Result<()> {
    loop {
        let mut state = &mut stream.state;
        match state {
            UserState::Anon => {
                let username = stream.prompt(b"username: ").await?;
                validate_username(&username)?;
                stream.state = UserState::Authenticating(username);
            }
            UserState::Authenticating(username) => {
                let password = stream.prompt(b"password: ").await?;
                validate_password(&password)?;
                stream.state = UserState::Authenticated;
            }
            UserState::Authenticated => return Ok(()),
        }
    }
}

// -----------------------------------------------------------------------------
//   - Authenticate user -
//   Authenticate the user, then move on to character creation
//   or character selection
// -----------------------------------------------------------------------------
pub async fn handle_connection(stream: TcpStream, router_tx: RouterSender) -> Result<()> {
    let mut stream = UserStream {
        stream: BufStream::new(stream),
        buffer: String::new(),
        state: UserState::Anon,
        router: router_tx,
    };

    let mut state = UserState::Anon;

    stream.write_line(GREETING).await;

    loop {
        match authenticate(&mut stream).await {
            Err(Error::Validation(err)) => stream.write_line(err.to_string().as_bytes()).await?,
            Err(err) => {
                log::error!("{err}");
                let _ = stream.write_line(format!("ERR: {err}").as_bytes()).await;
                return Ok(());
            }
            Ok(()) => break,
        }
    }

    crate::character::character_creation(stream).await
}

fn validate_username(username: &str) -> Result<()> {
    validation::username_len(username)?;
    validation::no_starting_space(username)?;
    Ok(())
}

fn validate_password(password: &str) -> Result<()> {
    validation::password_len(password)?;
    Ok(())
}
