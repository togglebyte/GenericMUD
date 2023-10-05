use std::collections::HashMap;
use std::time::Duration;

use crate::mpsc::{Receiver, Sender};
use crate::commands::Command;
use crate::player::PlayerId;
use crate::room::RoomId;

pub async fn start_router(rx: Receiver) {
    Router::new(rx).start().await;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Address {
    World,
    Player(PlayerId),
    Room(RoomId),
}

#[derive(Debug)]
pub enum Message {
    Command(Command, Address),
    Feedback(String, Address),
    Disconnect(PlayerId),
    Register(Sender, Address),
    Tick(Duration),
}

pub enum SenderId {
    World,
    Player(PlayerId),
}

#[derive(Debug)]
struct Router { 
    rx: Receiver,
    inner: HashMap<Address, Sender> 
}

impl Router {
    fn new(rx: Receiver) -> Self {
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

    async fn route_message(&mut self, msg: Message) {

        match msg {
            Message::Register(tx, addr) => {
                log::info!("registered {addr:?}");
                self.inner.insert(addr, tx);
            }
            // Command(cmd, address) => {
            // }
            _ => {} // Message::Command(
        }
    }
}
