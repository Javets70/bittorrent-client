use crate::bencode::value::BencodeValue;
use std::collections::HashMap;

pub struct File {
    pub length: usize,
    pub path: Vec<String>,
}

pub enum FilesInfo {
    SingleFile { length: usize },
    MultiFile { files: Vec<File> },
}

pub struct Info {
    pub name: String,
    pub piece_length: usize,
    pub pieces: Vec<[u8; 20]>,
    pub files_info: FilesInfo,
}

pub struct TorrentMetaInfo {
    pub announce: String,
    pub info: Info,
}

pub trait ToBencode {
    fn to_bencode_value(&self) -> BencodeValue;
}

impl ToBencode for Info {
    fn to_bencode_value(&self) -> BencodeValue {
        let mut dict = HashMap::new();

        dict.insert("name".to_string(), BencodeValue::String(self.name.clone()));
        dict.insert(
            "piece length".to_string(),
            BencodeValue::Integer(self.piece_length as i64),
        );

        let pieces_bytes: Vec<u8> = self
            .pieces
            .iter()
            .flat_map(|hash| hash.iter().copied())
            .collect();
        dict.insert("pieces".to_string(), BencodeValue::Bytes(pieces_bytes));

        match &self.files_info {
            FilesInfo::SingleFile { length } => {
                dict.insert("length".to_string(), BencodeValue::Integer(*length as i64));
            }
            FilesInfo::MultiFile { files } => {
                let files_list: Vec<BencodeValue> =
                    files.iter().map(|f| f.to_bencode_value()).collect();
                dict.insert("files".to_string(), BencodeValue::List(files_list));
            }
        }

        BencodeValue::Dictionary(dict)
    }
}

impl ToBencode for File {
    fn to_bencode_value(&self) -> BencodeValue {
        let mut dict = HashMap::new();

        dict.insert(
            "length".to_string(),
            BencodeValue::Integer(self.length as i64),
        );

        let path_list: Vec<BencodeValue> = self
            .path
            .iter()
            .map(|s| BencodeValue::String(s.clone()))
            .collect();
        dict.insert("path".to_string(), BencodeValue::List(path_list));

        BencodeValue::Dictionary(dict)
    }
}

impl TorrentMetaInfo {
    pub fn info_hash(&self) -> [u8; 20] {
        use crate::bencode::encoder;
        use sha1::{Digest, Sha1};

        let info_bencode = self.info.to_bencode_value();
        let bencode_bytes = encoder::encode(&info_bencode);

        let mut hasher = Sha1::new();
        hasher.update(&bencode_bytes);
        hasher.finalize().into()
    }

    // pub fn info_hash_urlencoded(&self) -> String {
    //     self.info_hash()
    //         .iter()
    //         .map(|byte| format!("%{:02X}", byte))
    //         .collect()
    // }

    pub fn total_size(&self) -> usize {
        match &self.info.files_info {
            FilesInfo::SingleFile { length } => *length,
            FilesInfo::MultiFile { files } => files.iter().map(|f| f.length).sum(),
        }
    }

    pub fn num_pieces(&self) -> usize {
        self.info.pieces.len()
    }
}
