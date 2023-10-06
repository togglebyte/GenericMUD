use std::marker::Unpin;

use bytes::Bytes;
use flume::Receiver;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};

use crate::character::Character;
use crate::commands::parse;
use crate::mpsc::{unbounded_client, ClientSender, RouterSender};
use crate::player::PlayerId;
use crate::router::{Address, RouterMessage};
use crate::user::UserStream;

#[derive(Debug)]
pub enum Message {
    Disconnect,
    Payload(Bytes),
}

pub async fn go_game_go(user: UserStream, character: Character) {
    let (reader, writer) = user.stream.into_inner().into_split();

    let player_id = PlayerId::next();
    let address = Address::Player(player_id);

    let (tx, rx) = unbounded_client();

    // Register the new player with the router
    user.router
        .send_async(RouterMessage::Register(tx, address))
        .await;

    let writer = ClientWrite { writer, rx };

    let reader = ClientRead {
        reader: BufReader::new(reader),
        router: user.router,
        input: String::new(),
        address,
        player_id,
    };

    writer.start();
    reader.start();
}

// -----------------------------------------------------------------------------
//   - Client write half -
// -----------------------------------------------------------------------------
struct ClientWrite<W> {
    writer: W,
    rx: Receiver<Message>,
}

impl<W: AsyncWriteExt + Unpin + Send + 'static> ClientWrite<W> {
    fn start(mut self) {
        tokio::spawn(async move {
            while let Ok(msg) = self.rx.recv_async().await {
                match msg {
                    Message::Disconnect => break,
                    Message::Payload(bytes) => {
                        if let Err(err) = self.writer.write_all(&*bytes).await {
                            log::error!("{err}");
                            break;
                        }
                    }
                }
            }
        });
    }
}

// -----------------------------------------------------------------------------
//   - Client read half -
// -----------------------------------------------------------------------------
struct ClientRead<R> {
    reader: R,
    router: RouterSender,
    input: String,
    player_id: PlayerId,
    address: Address,
}

impl<R: AsyncBufReadExt + Unpin + Send + 'static> ClientRead<R> {
    fn start(mut self) {
        tokio::spawn(async move {
            loop {
                self.input.clear();
                if let Err(e) = self.reader.read_line(&mut self.input).await {
                    log::error!("{e}");
                }

                let Some(command) = parse(self.player_id, &self.input) else {
                    continue;
                };

                self.router
                    .send_async(RouterMessage::Command {
                        command,
                        sender: self.address,
                    })
                    .await;
            }
        });
    }
}
