use std::sync::atomic::{AtomicUsize, Ordering};

use player::PlayerId;
use router::{start_router, Address, Message};
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use world::start_the_universe;

mod commands;
mod mpsc;
mod player;
mod room;
mod router;
mod user;
mod world;

// Commands
// Rooms
// NPC
// Mobs
// Users / Players
// Combat
// Router

pub fn next_id() -> usize {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    NEXT.fetch_add(1, Ordering::Relaxed)
}

pub async fn serve() {
    // Start routing messages
    let (router_tx, router_rx) = mpsc::unbounded();
    tokio::spawn(start_router(router_rx));

    // Setup the world
    let (world_tx, world_rx) = mpsc::unbounded();
    tokio::spawn(start_the_universe(world_rx, router_tx.clone()));
    router_tx
        .send_async(Message::Register(world_tx, Address::World))
        .await;

    // Incomming connections
    let mut listener = TcpListener::bind("127.0.0.1:1234").await.unwrap();

    loop {
        if let Ok((s, _)) = listener.accept().await {
            tokio::spawn(user::handle_connection(s, router_tx.clone()));
        }
    }
}
