#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoomId(usize);

impl RoomId {
    pub fn next() -> RoomId {
        RoomId(crate::next_id())
    }
}
