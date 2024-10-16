pub use crate::rpc::activity::Activity;

pub struct Packet<'a> {
    pub pid: u32,
    pub activity: Option<&'a Activity<'a>>,
}

impl<'a> Packet<'a> {
    pub fn new(pid: u32, activity: Option<&'a Activity>) -> Packet<'a> {
        Packet { pid, activity }
    }
}
