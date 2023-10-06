use std::collections::HashMap;
use std::time::Duration;

use crate::mpsc::{RouterReceiver, RouterSender, ClientSender};
use crate::commands::Command;
use crate::player::PlayerId;
use crate::room::RoomId;

pub async fn start_router(rx: RouterReceiver) {
    Router::new(rx).start().await;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Address {
    World,
    Player(PlayerId),
    Room(RoomId),
}

#[derive(Debug)]
pub enum RouterMessage {
    Command { command: Command, sender: Address },
    Disconnect(PlayerId),
    Register(ClientSender, Address),
    Tick(Duration),
}

pub enum SenderId {
    World,
    Player(PlayerId),
}

#[derive(Debug)]
struct Router { 
    rx: RouterReceiver,
    inner: HashMap<Address, ClientSender> 
}

impl Router {
    fn new(rx: RouterReceiver) -> Self {
        Self {
            rx,
            inner: HashMap::new() 
        }
    }

    async fn start(mut self) {
        log::info!("starting router...");

        while let Ok(message) = self.rx.recv_async().await {
            self.route_message(message).await;
        }
    }

    async fn route_message(&mut self, msg: RouterMessage) {

        match msg {
            RouterMessage::Register(tx, addr) => {
                log::info!("registered {addr:?}");
                self.inner.insert(addr, tx);
            }
            RouterMessage::Command { command, sender } => match command {
                Command::Tell(data) => {
                    //
                }
                // Command::Say(say) => {}
            }
            _ => {}
        }
    }
}
