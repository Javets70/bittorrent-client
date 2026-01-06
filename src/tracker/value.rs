use std::net;

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Started,
    Completed,
    Stopped,
}

impl Event {
    pub fn as_str(&self) -> &str {
        match self {
            Event::Started => "started",
            Event::Completed => "completed",
            Event::Stopped => "stopped",
        }
    }
}

#[derive(Debug)]
pub struct TrackerRequest {
    pub announce_url: String,
    pub info_hash: [u8; 20],
    pub peer_id: [u8; 20],
    pub ip: Option<net::Ipv4Addr>,
    pub port: u16,
    pub uploaded: u64,
    pub downloaded: u64,
    pub left: u64,
    pub compact: bool,
    pub event: Option<Event>,
}

impl TrackerRequest {
    pub fn generate_peer_id() -> [u8; 20] {
        use rand::prelude::*;
        let mut peer_id = *b"-RS0001-000000000000"; // Your client prefix
        let mut rng = rand::rng();

        for i in 8..20 {
            peer_id[i] = rng.random::<u8>();
        }

        peer_id
    }
    fn url_encode_bytes(bytes: &[u8]) -> String {
        bytes.iter().map(|&b| format!("%{:02X}", b)).collect()
    }

    pub fn build_url(&self) -> String {
        let mut url = self.announce_url.clone();
        url.push('?');

        url.push_str(&format!(
            "info_hash={}",
            Self::url_encode_bytes(&self.info_hash)
        ));
        url.push_str(&format!(
            "&peer_id={}",
            Self::url_encode_bytes(&self.peer_id)
        ));
        url.push_str(&format!("&port={}", self.port));
        url.push_str(&format!("&uploaded={}", self.uploaded));
        url.push_str(&format!("&downloaded={}", self.downloaded));
        url.push_str(&format!("&left={}", self.left));
        url.push_str(&format!("&compact={}", if self.compact { 1 } else { 0 }));

        if let Some(event) = &self.event {
            url.push_str(&format!("&event={}", event.as_str()));
        }

        if let Some(ip) = &self.ip {
            url.push_str(&format!("&ip={}", ip));
        }

        url
    }
}

#[derive(Debug, Clone)]
pub struct Peer {
    pub id: Option<Vec<u8>>,
    pub ip: net::Ipv4Addr,
    pub port: u16,
}

#[derive(Debug)]
pub struct TrackerResponse {
    pub interval: u32,
    pub peers: Vec<Peer>,
}
