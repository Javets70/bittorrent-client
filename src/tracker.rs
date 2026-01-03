pub enum Event {
    Started,
    Completed,
    Stopped,
}

pub struct TrackerRequest {
    pub peer_id: Vec<[u8; 20]>,
    pub info_hash: Vec<[u8; 20]>,
    pub port: u16,
    pub uploaded: usize,
    pub downloaded: usize,
    pub left: usize,
    pub event: Event,
    // pub ip
}
