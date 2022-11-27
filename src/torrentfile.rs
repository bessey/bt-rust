// #[derive(Debug, Deserialize)]
// pub struct Node(String, i64);

// #[derive(Debug, Deserialize)]
// pub struct File {
//     path: Vec<String>,
//     length: i64,
//     #[serde(default)]
//     md5sum: Option<String>,
// }

use crate::bencode::bencode_decode;

#[derive(Debug)]
pub struct Info {
    name: String,
    // pieces: Vec<u8>,
    // piece_length: i64,
    // md5sum: Option<String>,
    // length: Option<i64>,
    // files: Option<Vec<File>>,
    // private: Option<u8>,
    // path: Option<Vec<String>>,
    // root_hash: Option<String>,
}

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

#[derive(Debug)]
pub struct Torrent {
    info: Info,
    // pub fn debug(&self) {
    //     println!("name:\t\t{}", self.info.name);
    //     println!("announce:\t{:?}", self.announce);
    //     println!("nodes:\t\t{:?}", self.nodes);
    //     if let &Some(ref al) = &self.announce_list {
    //         for a in al {
    //             println!("announce list:\t{}", a[0]);
    //         }
    //     }
    //     println!("creation date:\t{:?}", self.creation_date);
    //     println!("comment:\t{:?}", self.comment);
    //     println!("created by:\t{:?}", self.created_by);
    //     println!("encoding:\t{:?}", self.encoding);
    //     println!("info byes:\t{:?}", self.info_bytes);
    //     println!("piece length:\t{:?}", self.info.piece_length);
    //     println!("private:\t{:?}", self.info.private);
    //     println!("root hash:\t{:?}", self.info.root_hash);
    //     println!("md5sum:\t\t{:?}", self.info.md5sum);
    //     println!("path:\t\t{:?}", self.info.path);
    //     if let &Some(ref files) = &self.info.files {
    //         for f in files {
    //             println!("file path:\t{:?}", f.path);
    //             println!("file length:\t{}", f.length);
    //             println!("file md5sum:\t{:?}", f.md5sum);
    //         }
    //     }
    // }
}

pub fn decode_torrent(metainfo: Vec<u8>) -> Torrent {
    let info = Info {
        name: "test".to_string(),
    };

    // Look at the first character
    // Call implementation for that character

    let result = bencode_decode(&metainfo);
    println!("Value: {:?}", result);

    return Torrent { info: info };
}
