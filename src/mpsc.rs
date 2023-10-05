use crate::router::Message;

pub type Sender = flume::Sender<Message>;
pub type Receiver = flume::Receiver<Message>;

pub fn unbounded() -> (Sender, Receiver) {
    flume::unbounded()
}
