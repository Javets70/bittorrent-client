use std::collections::HashMap;
use std::error::Error;
use std::fs;

use crate::bencode::helper::{get_bytes, get_dict, get_int, get_list, get_string};
use crate::bencode::parser::parse_value;
use crate::bencode::value::BencodeValue;

use super::value::{File, FilesInfo, Info, TorrentMetaInfo};

pub fn parse_torrent_file(path: &str) -> Result<TorrentMetaInfo, Box<dyn Error>> {
    let contents = fs::read(path)?;
    let (bencode_value, _) = parse_value(&contents)?;
    torrent_from_bencode(&bencode_value)
}

fn get_files_info(dict: &HashMap<String, BencodeValue>) -> Result<FilesInfo, Box<dyn Error>> {
    // There is also a key 'length' or a key 'files', but not both or neither.
    // If length is present then the download represents a single file,
    // otherwise it represents a set of files which go in a directory structure.
    let has_length = dict.contains_key("length");
    let has_files = dict.contains_key("files");

    match (has_length, has_files) {
        (true, false) => {
            let length = get_int(dict, "length")? as usize;
            Ok(FilesInfo::SingleFile { length })
        }
        (false, true) => {
            let files = parse_files_list(dict)?;
            Ok(FilesInfo::MultiFile { files })
        }
        _ => Err("Must have exactly one of 'length' or 'files'".into()),
    }
}

pub fn parse_files_list(dict: &HashMap<String, BencodeValue>) -> Result<Vec<File>, Box<dyn Error>> {
    get_list(dict, "files")?
        .iter()
        .map(|file_value| {
            let file_dict = file_value.as_dict()?;
            let length = get_int(file_dict, "length")? as usize;
            let path = get_list(file_dict, "path")?
                .iter()
                .map(|p| p.as_string().map(String::from))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(File { length, path })
        })
        .collect()
}

pub fn torrent_from_bencode(input: &BencodeValue) -> Result<TorrentMetaInfo, Box<dyn Error>> {
    let bencode_dict = input.as_dict()?;

    let announce = get_string(bencode_dict, "announce")?;
    let info_dict = get_dict(bencode_dict, "info")?;

    let name = get_string(info_dict, "name")?;
    let piece_length: usize = get_int(info_dict, "piece length")? as usize;
    let pieces_bytes = get_bytes(info_dict, "pieces")?;
    let pieces: Vec<[u8; 20]> = pieces_bytes
        .chunks(20)
        .map(|chunk| chunk.try_into().map_err(|_| "Invalid piece length"))
        .collect::<Result<Vec<_>, _>>()?;

    let files_info = get_files_info(info_dict)?;

    Ok(TorrentMetaInfo {
        announce,
        info: Info {
            name,
            piece_length,
            pieces,
            files_info,
        },
    })
}
