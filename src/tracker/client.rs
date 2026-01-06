use super::value::{Peer, TrackerRequest, TrackerResponse};
use crate::bencode::helper::{get_bytes, get_int, get_list, get_string};
use crate::bencode::parser::parse_value;
use crate::bencode::value::BencodeValue;
use std::error::Error;

pub struct TrackerClient;

impl TrackerClient {
    pub fn query_tracker(
        request: &TrackerRequest,
    ) -> Result<TrackerResponse, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::builder().build()?;
        let url = request.build_url();

        let response = client.get(&url).send()?;
        let response_bytes = response.bytes()?;

        parse_tracker_response(&response_bytes)
    }
}

fn parse_tracker_response(data: &[u8]) -> Result<TrackerResponse, Box<dyn Error>> {
    let (bencode_value, _) = parse_value(data)?;
    let dict = bencode_value.as_dict()?;

    let interval = get_int(dict, "interval")? as u32;

    let peers_value = get_list(dict, "peers")?;
    let peers = parse_peers(peers_value)?;

    Ok(TrackerResponse { interval, peers })
}

fn parse_peers(peers_data: &Vec<BencodeValue>) -> Result<Vec<Peer>, Box<dyn Error>> {
    use std::net::Ipv4Addr;
    let mut peers = Vec::new();

    for peer_value in peers_data {
        let peer_dict = peer_value.as_dict()?;

        let peer_id = match get_bytes(peer_dict, "peer id") {
            Ok(bytes) => Some(bytes.to_vec()),
            Err(_) => None,
        };
        let ip_str = get_string(peer_dict, "ip")?;
        let ip = ip_str.parse::<Ipv4Addr>()?;

        let port = get_int(peer_dict, "port")? as u16;

        peers.push(Peer {
            id: peer_id,
            ip,
            port,
        });
    }

    Ok(peers)
}
