use crate::router::RouterMessage;
use crate::client;

pub type ClientSender = flume::Sender<client::Message>;
pub type ClientReceiver = flume::Receiver<client::Message>;

pub type RouterSender = flume::Sender<RouterMessage>;
pub type RouterReceiver = flume::Receiver<RouterMessage>;

pub fn unbounded_client() -> (ClientSender, ClientReceiver) {
    flume::unbounded()
}

pub fn unbounded_router() -> (RouterSender, RouterReceiver) {
    flume::unbounded()
}
