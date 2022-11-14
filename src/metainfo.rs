// extern crate serde;
// extern crate serde_bencode;
// use serde::Deserialize;

use std::fs::File;
use std::io::Read;

pub fn read_torrent_file(path: &str) -> std::io::Result<Vec<u8>> {
    let mut f = File::open(path)?;
    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer)?;
    return Ok(buffer);
}

// #[derive(Debug, Deserialize)]
// pub struct Node(String, i64);

// #[derive(Debug, Deserialize)]
// pub struct File {
//     path: Vec<String>,
//     length: i64,
//     #[serde(default)]
//     md5sum: Option<String>,
// }

// #[derive(Debug, Deserialize)]
// pub struct Info {
//     name: String,
//     #[serde(with = "serde_bytes")]
//     pieces: Vec<u8>,
//     #[serde(rename = "piece length")]
//     piece_length: i64,
//     #[serde(default)]
//     md5sum: Option<String>,
//     #[serde(default)]
//     length: Option<i64>,
//     #[serde(default)]
//     files: Option<Vec<File>>,
//     #[serde(default)]
//     private: Option<u8>,
//     #[serde(default)]
//     path: Option<Vec<String>>,
//     #[serde(default)]
//     #[serde(rename = "root hash")]
//     root_hash: Option<String>,
// }

// #[derive(Debug, Deserialize)]
// pub struct Torrent {
//     #[serde(with = "serde_bytes")]
//     #[serde(rename = "info")]
//     #[serde(skip_serializing)]
//     info_bytes: Vec<u8>,
//     info: Info,
//     #[serde(default)]
//     announce: Option<String>,
//     #[serde(default)]
//     nodes: Option<Vec<Node>>,
//     #[serde(default)]
//     encoding: Option<String>,
//     #[serde(default)]
//     #[serde(rename = "announce-list")]
//     announce_list: Option<Vec<Vec<String>>>,
//     #[serde(default)]
//     #[serde(rename = "creation date")]
//     creation_date: Option<i64>,
//     #[serde(rename = "comment")]
//     comment: Option<String>,
//     #[serde(default)]
//     #[serde(rename = "created by")]
//     created_by: Option<String>,
// }

// impl Torrent {
//     pub fn announce(&self) -> String {
//         return match self {
//             Torrent {
//                 announce: Some(url),
//                 ..
//             } => url.clone(),
//             Torrent {
//                 announce_list: Some(urls),
//                 ..
//             } => announce_list_first(urls),
//             Torrent { .. } => panic!("No Announce available"),
//         };
//     }

//     pub fn debug(&self) {
//         println!("name:\t\t{}", self.info.name);
//         println!("announce:\t{:?}", self.announce);
//         println!("nodes:\t\t{:?}", self.nodes);
//         if let &Some(ref al) = &self.announce_list {
//             for a in al {
//                 println!("announce list:\t{}", a[0]);
//             }
//         }
//         println!("creation date:\t{:?}", self.creation_date);
//         println!("comment:\t{:?}", self.comment);
//         println!("created by:\t{:?}", self.created_by);
//         println!("encoding:\t{:?}", self.encoding);
//         println!("info byes:\t{:?}", self.info_bytes);
//         println!("piece length:\t{:?}", self.info.piece_length);
//         println!("private:\t{:?}", self.info.private);
//         println!("root hash:\t{:?}", self.info.root_hash);
//         println!("md5sum:\t\t{:?}", self.info.md5sum);
//         println!("path:\t\t{:?}", self.info.path);
//         if let &Some(ref files) = &self.info.files {
//             for f in files {
//                 println!("file path:\t{:?}", f.path);
//                 println!("file length:\t{}", f.length);
//                 println!("file md5sum:\t{:?}", f.md5sum);
//             }
//         }
//     }
// }
