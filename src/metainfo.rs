extern crate serde;
extern crate serde_bencode;
use sha1::{Digest, Sha1};

use serde::Deserialize;
use serde::Serialize;
use serde_bencode::de;
use serde_bencode::ser;
use std::fs::File as FileIO;
use std::io::Read;
use std::io::Write;

#[derive(Debug, Deserialize)]
pub struct Node(String, i64);

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    path: Vec<String>,
    length: i64,
    #[serde(default)]
    md5sum: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    // Common
    #[serde(with = "serde_bytes")]
    pieces: Vec<u8>,
    #[serde(rename = "piece length")]
    piece_length: i64,
    #[serde(default)]
    private: Option<u8>,

    // Single File Mode
    #[serde(default)]
    name: String,
    #[serde(default)]
    length: Option<i64>,
    #[serde(default)]
    md5sum: Option<String>,

    // Multiple File mode
    #[serde(default)]
    files: Option<Vec<File>>,

    // Misc
    #[serde(skip_serializing)]
    root_hash: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Torrent {
    info: Info,
    #[serde(default)]
    announce: Option<String>,
    #[serde(default)]
    nodes: Option<Vec<Node>>,
    #[serde(default)]
    encoding: Option<String>,
    #[serde(default)]
    #[serde(rename = "announce-list")]
    announce_list: Option<Vec<Vec<String>>>,
    #[serde(default)]
    #[serde(rename = "creation date")]
    creation_date: Option<i64>,
    #[serde(rename = "comment")]
    comment: Option<String>,
    #[serde(default)]
    #[serde(rename = "created by")]
    created_by: Option<String>,
}

impl Torrent {
    pub fn announce(&self) -> String {
        return match self {
            Torrent {
                announce: Some(url),
                ..
            } => url.clone(),
            Torrent {
                announce_list: Some(urls),
                ..
            } => announce_list_first(urls),
            Torrent { .. } => panic!("No Announce available"),
        };
    }

    pub fn debug(&self) {
        println!("name:\t\t{}", self.info.name);
        println!("announce:\t{:?}", self.announce);
        println!("nodes:\t\t{:?}", self.nodes);
        if let &Some(ref al) = &self.announce_list {
            for a in al {
                println!("announce list:\t{}", a[0]);
            }
        }
        println!("creation date:\t{:?}", self.creation_date);
        println!("comment:\t{:?}", self.comment);
        println!("created by:\t{:?}", self.created_by);
        println!("encoding:\t{:?}", self.encoding);
        println!("piece length:\t{:?}", self.info.piece_length);
        println!("private:\t{:?}", self.info.private);
        println!("md5sum:\t\t{:?}", self.info.md5sum);
        println!("root hash:\t{:?}", self.info.root_hash);
        if let &Some(ref files) = &self.info.files {
            for f in files {
                println!("file path:\t{:?}", f.path);
                println!("file length:\t{}", f.length);
                println!("file md5sum:\t{:?}", f.md5sum);
            }
        }
    }
}

pub fn read_torrent_file(path: &str) -> Result<Torrent, String> {
    let mut encoded = Vec::new();
    let mut file = FileIO::open(path).or(Err("File couldn't be opened"))?;
    file.read_to_end(&mut encoded)
        .or(Err("File couldn't be read"))?;
    let mut torrent = de::from_bytes::<Torrent>(&encoded).or(Err("Torrent file invalid"))?;
    let info_bytes = ser::to_bytes(&torrent.info).or(Err("Info invalid"))?;
    FileIO::create("metainfo")
        .or(Err("Couldn't create file"))?
        .write(&info_bytes)
        .or(Err("Couldn't write file"))?;
    let mut hasher = Sha1::new();
    hasher.update(info_bytes);
    let hex_hash = base16ct::lower::encode_string(&hasher.finalize());
    torrent.info.root_hash = Some(hex_hash);

    return Ok(torrent);
}

fn announce_list_first(urls: &Vec<Vec<String>>) -> String {
    return urls.first().unwrap().first().unwrap().clone();
}
