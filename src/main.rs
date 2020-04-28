use bendy::{
    decoding::{Error, FromBencode, Object, ResultExt},
    encoding::AsString,
};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct MetaInfo {
    announce: String,
    comment: String,
    url_list: Vec<String>,
    info: Info,
}

#[derive(Debug)]
struct Info {
    name: String,
    piece_length: u32,
    pieces: PieceSet,
}

#[derive(Debug)]
struct Sha(Vec<u8>);

#[derive(Debug)]
struct PieceSet(Vec<Sha>);

impl FromBencode for PieceSet {
    const EXPECTED_RECURSION_DEPTH: usize = 0;

    fn decode_bencode_object(object: Object) -> Result<Self, Error> {
        let content = AsString::decode_bencode_object(object)?;
        let chars = content.0;
        Ok(PieceSet(
            chars.chunks(20).map(|i| Sha(i.to_vec())).collect(),
        ))
    }
}
fn main() {
    let target = "./archlinux-2020.01.01-x86_64.iso.torrent";
    match read_torrent_file(target) {
        Err(e) => println!("Error {:?}", e),
        Ok(decoded) => println!("File contains {:?}", decoded),
    }
}

impl FromBencode for MetaInfo {
    fn decode_bencode_object(object: Object) -> Result<Self, Error> {
        let mut announce = None;
        let mut comment = None;
        let mut url_list = None;
        let mut info = None;
        let mut dict = object.try_into_dictionary()?;
        while let Some(pair) = dict.next_pair()? {
            match pair {
                (b"announce", value) => {
                    announce = String::decode_bencode_object(value)
                        .context("announce")
                        .map(Some)?;
                }
                (b"comment", value) => {
                    comment = String::decode_bencode_object(value)
                        .context("comment")
                        .map(Some)?;
                }
                (b"url-list", value) => {
                    url_list = Vec::decode_bencode_object(value)
                        .context("url_list")
                        .map(Some)?;
                }
                (b"info", value) => {
                    info = Info::decode_bencode_object(value)
                        .context("info")
                        .map(Some)?;
                }
                (unknown_field, _) => {
                    println!("Unexpected field {:?}", std::str::from_utf8(unknown_field))
                }
            }
        }
        let announce = announce.ok_or_else(|| Error::missing_field("announce"))?;
        let comment = comment.ok_or_else(|| Error::missing_field("comment"))?;
        let url_list = url_list.ok_or_else(|| Error::missing_field("url_list"))?;
        let info = info.ok_or_else(|| Error::missing_field("info"))?;

        Ok(MetaInfo {
            announce,
            comment,
            url_list,
            info,
        })
    }
}

impl FromBencode for Info {
    fn decode_bencode_object(object: Object) -> Result<Self, Error> {
        let mut name = None;
        let mut piece_length = None;
        let mut pieces = None;
        let mut dict = object.try_into_dictionary()?;
        while let Some(pair) = dict.next_pair()? {
            match pair {
                (b"name", value) => {
                    name = String::decode_bencode_object(value)
                        .context("name")
                        .map(Some)?;
                }
                (b"piece length", value) => {
                    piece_length = u32::decode_bencode_object(value)
                        .context("piece_length")
                        .map(Some)?;
                }
                (b"pieces", value) => {
                    pieces = PieceSet::decode_bencode_object(value)
                        .context("pieces")
                        .map(Some)?;
                }
                (unknown_field, _) => {
                    println!("Unexpected field {:?}", std::str::from_utf8(unknown_field))
                }
            }
        }
        let name = name.ok_or_else(|| Error::missing_field("name"))?;
        let pieces = pieces.ok_or_else(|| Error::missing_field("pieces"))?;
        let piece_length = piece_length.ok_or_else(|| Error::missing_field("piece_length"))?;

        Ok(Info {
            name,
            piece_length,
            pieces,
        })
    }
}

fn read_torrent_file(path: &str) -> Result<MetaInfo, Error> {
    let mut file = File::open(path)?;
    let mut encoded = Vec::new();
    file.read_to_end(&mut encoded)?;
    let decoded = MetaInfo::from_bencode(&encoded)?;
    Ok(decoded)
}
