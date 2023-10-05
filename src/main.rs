use tokio::net::{TcpStream, TcpListener};
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    genericmud::serve().await;
}
