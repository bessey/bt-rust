mod bencode;
mod metainfo;
mod torrentfile;

use crate::{bencode::decode, torrentfile::decode_torrent};

fn main() {
    let target = "./archlinux-2020.01.01-x86_64.iso.torrent";
    let metainfo = match metainfo::read_torrent_file(target) {
        Err(error) => panic!("Errorz {:?}", error),
        Ok(decoded) => decoded,
    };

    println!("Torrent bytes: {}", metainfo.len());

    let decoded = decode(&metainfo);
    println!("{:?}", decoded);

    // let torrent = decode_torrent(metainfo);
    // println!("{:?}", torrent);
}
