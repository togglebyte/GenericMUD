#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerId(usize);

impl PlayerId {
    pub fn next() -> PlayerId {
        PlayerId(crate::next_id())
    }
}
