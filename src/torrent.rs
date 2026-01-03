use std::collections::HashMap;
use std::error::Error;
use std::fs;

use crate::bencode::{self, BencodeValue};

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

pub fn parse_torrent_file(path: &str) -> Result<TorrentMetaInfo, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let (bencode_value, _) = bencode::parse_value(&contents)?;
    torrent_from_bencode(&bencode_value)
}

fn get_files_info(dict: &HashMap<String, BencodeValue>) -> Result<FilesInfo, Box<dyn Error>> {
    // There is also a key 'length' or a key 'files', but not both or neither.
    // If length is present then the download represents a single file,
    // otherwise it represents a set of files which go in a directory structure.
    let has_length = dict.contains_key("length");
    let has_files = dict.contains_key("files");

    if !(has_length ^ has_files) {
        return Err("Must have either 'length' or 'files', not both or neither".into());
    }

    if has_length {
        let length = bencode::get_int(dict, "length")? as usize;
        return Ok(FilesInfo::SingleFile { length });
    }

    // Multi-file case
    let files_list = bencode::get_list(dict, "files")?;
    let files: Result<Vec<File>, Box<dyn Error>> = files_list
        .iter()
        .map(|file_value| {
            let file_dict = match file_value {
                BencodeValue::Dict(d) => d,
                _ => return Err("File entry must be a dictionary".into()),
            };

            let length = bencode::get_int(file_dict, "length")? as usize;

            let path_list = bencode::get_list(file_dict, "path")?;
            let path: Vec<String> = path_list
                .iter()
                .map(|p| match p {
                    BencodeValue::String(s) => Ok(s.clone()),
                    _ => Err("Path component must be a string".into()),
                })
                .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

            Ok(File { length, path })
        })
        .collect();

    Ok(FilesInfo::MultiFile { files: files? })
}

pub fn torrent_from_bencode(
    input: &bencode::BencodeValue,
) -> Result<TorrentMetaInfo, Box<dyn Error>> {
    let bencode_dict = match input {
        bencode::BencodeValue::Dict(value) => value,
        _ => return Err("Expected BencodeValue::Dict".into()),
    };

    let announce = bencode::get_string(bencode_dict, "announce")?;
    let info_dict = bencode::get_dict(bencode_dict, "info")?;

    let name = bencode::get_string(info_dict, "name")?;
    let piece_length: usize = bencode::get_int(info_dict, "piece length")? as usize;
    let pieces_bytes = bencode::get_bytes(info_dict, "pieces")?;
    let pieces: Vec<[u8; 20]> = pieces_bytes
        .chunks(20)
        .map(|chunk| chunk.try_into().map_err(|_| "Invalid piece length"))
        .collect::<Result<Vec<_>, _>>()?;

    let files_info = get_files_info(info_dict)?;

    Ok(TorrentMetaInfo {
        announce: announce.to_string(),
        info: Info {
            name: name.to_string(),
            piece_length,
            pieces,
            files_info,
        },
    })
}
