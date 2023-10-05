use std::time::{Duration, Instant};

use crate::router::Message;
use crate::mpsc::{Receiver, Sender};

pub async fn start_the_universe(rx: Receiver, router: Sender) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        let mut now = Instant::now();

        loop {
            interval.tick().await;
            router.send_async(Message::Tick(now.elapsed())).await;
            now = Instant::now();
        }
    });

    log::info!("started the universe!");

    while let Ok(msg) = rx.recv_async().await {
        log::info!("universe: {msg:?}");
    }
}
