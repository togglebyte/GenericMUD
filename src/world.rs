use std::time::{Duration, Instant};

use crate::mpsc::{unbounded_client, RouterSender};
use crate::router::{RouterMessage, Address};

pub async fn start_the_universe(router: RouterSender) {
    // World sender / receiver pair
    let (tx, rx) = unbounded_client();

    // Register the world address with the router
    router
        .send_async(RouterMessage::Register(tx, Address::World))
        .await;

    // Start the tick
    tokio::spawn(async move {
        // Start the timer
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        let mut now = Instant::now();

        loop {
            interval.tick().await;
            router.send_async(RouterMessage::Tick(now.elapsed())).await;
            now = Instant::now();
        }
    });

    log::info!("started the universe!");

    while let Ok(msg) = rx.recv_async().await {
        log::info!("universe: {msg:?}");
        // progress combat
        // respawns
        // update health for entities resting
    }
}
