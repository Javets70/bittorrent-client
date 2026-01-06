use super::error::{HandshakeError, PeerHandshakeError};
use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Debug, Clone)]
pub struct Handshake {
    // - Length byte (1 byte): Always 19 (the length of the protocol string)
    // - Protocol string (19 bytes): Always "BitTorrent protocol"
    // - Reserved bytes (8 bytes): All zeros, reserved for future extensions
    // - Info hash (20 bytes): The SHA1 hash of the torrent's info section
    // - Peer ID (20 bytes): A unique identifier for your client
    pub length: u8,
    pub protocol: [u8; 19],
    pub reserved: [u8; 8],
    pub info_hash: [u8; 20],
    pub peer_id: [u8; 20],
}

impl Handshake {
    pub fn to_bytes(&self) -> [u8; 68] {
        let mut bytes: [u8; 68] = [0; 68];

        bytes[0] = self.length;
        bytes[1..20].copy_from_slice(&self.protocol);
        bytes[20..28].copy_from_slice(&self.reserved);
        bytes[28..48].copy_from_slice(&self.info_hash);
        bytes[48..68].copy_from_slice(&self.peer_id);

        bytes
    }

    pub fn from_bytes(
        bytes: &[u8],
        expected_info_hash: &[u8; 20],
        own_peer_id: &[u8; 20],
    ) -> Result<Handshake, HandshakeError> {
        if bytes.len() != 68 {
            return Err(HandshakeError::InvalidLength);
        }

        if bytes[0] != 19 {
            return Err(HandshakeError::InvalidProtocolLength(bytes[0]));
        }

        if &bytes[1..20] != b"BitTorrent protocol" {
            return Err(HandshakeError::InvalidProtocolString);
        }

        let mut reserved = [0u8; 8];
        reserved.copy_from_slice(&bytes[20..28]);

        let mut info_hash = [0u8; 20];
        info_hash.copy_from_slice(&bytes[28..48]);

        if &info_hash != expected_info_hash {
            return Err(HandshakeError::InfoHashMismatch);
        }

        let mut peer_id = [0u8; 20];
        peer_id.copy_from_slice(&bytes[48..68]);

        if &peer_id == own_peer_id {
            return Err(HandshakeError::SelfConnection);
        }

        Ok(Handshake {
            length: 19,
            protocol: *b"BitTorrent protocol",
            reserved,
            info_hash,
            peer_id,
        })
    }

    fn connect_to_peer(
        peer: &crate::tracker::value::Peer,
    ) -> Result<TcpStream, PeerHandshakeError> {
        let addr = format!("{}:{}", peer.ip, peer.port);
        let stream = TcpStream::connect(addr)?;

        Ok(stream)
    }

    pub fn perform_handshake(
        stream: &mut TcpStream,
        info_hash: &[u8; 20],
        own_peer_id: &[u8; 20],
    ) -> Result<Handshake, PeerHandshakeError> {
        let request_handshake = Handshake {
            length: 19,
            protocol: *b"BitTorrent protocol",
            reserved: [0u8; 8],
            info_hash: *info_hash,
            peer_id: *own_peer_id,
        };

        stream.write_all(&request_handshake.to_bytes())?;

        let mut response_buf = [0u8; 68];
        stream.read_exact(&mut response_buf)?;

        let response_handshake = Handshake::from_bytes(&response_buf, info_hash, own_peer_id)?;

        Ok(response_handshake)
    }
}



// This data is recieved in stages , first the length, then message_id and payload
//  1. Length Prefix (4 bytes): This is a 4-byte number (u32) that tells you the length of the
//     rest of the message (ID + Payload). It is always encoded in Big-Endian.
//  2. Message ID (1 byte): A single byte that tells you the type of message. For example, ID 5
//     is a bitfield message.
//  3. Payload (variable size): The actual data for the message. This can be empty. Its size is
//     Length - 1
pub enum PeerMessage {
    KeepAlive,
    Choke,
    Unchoke,
    Interested,
    NotInterested,
    Have {
        piece_index: u32,
    },
    Bitfield(Vec<u8>),
    Request {
        index: u32,
        begin: u32,
        length: u32,
    },
    Piece {
        index: u32,
        begin: u32,
        block: Vec<u8>,
    },
    Cancel {
        index: u32,
        begin: u32,
        length: u32,
    },
    Unknown {
        id: u8,
        payload: Vec<u8>,
    },
}

pub enum PeerMessageError {
    IOError(std::io::Error),
}

impl From<std::io::Error> for PeerMessageError{
    fn from(value: std::io::Error) -> Self {
        PeerMessageError::IOError(value)
    }
}

impl PeerMessage {
    pub fn read_peer_message(stream: &mut TcpStream) -> Result<PeerMessage, PeerMessageError> {
        let mut len_bytes = [0u8;4];
        stream.read_exact(&mut len_bytes)?;

        let message_len = u32::from_be_bytes(len_bytes);

        if message_len == 0{
            return Ok(PeerMessage::KeepAlive);
        }

        let mut payload_buffer = vec![0u8;message_len as usize];
        stream.read_exact(&mut payload_buffer)?;

        let message_id = payload_buffer[0];
        let payload = &payload_buffer[1..];


        match message_id{
            0 => {
                return Ok(PeerMessage::Choke)
            }
            2 => {
                return Ok(PeerMessage::Interested)
            }
            5 => {
                return Ok(PeerMessage::Bitfield(payload.to_vec()))
            }
            // 7 => {
            //     return Ok(PeerMessage::Piece { index: (), begin: (), block: () })
            // }
            _ => {
                return Ok(PeerMessage::Unknown { id: message_id, payload: payload.to_vec()})
            }
        }
    }
}
