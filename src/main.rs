mod metainfo;

fn main() {
    let target = "./archlinux-2020.01.01-x86_64.iso.torrent";
    match metainfo::read_torrent_file(target) {
        Err(e) => println!("Error {:?}", e),
        Ok(decoded) => println!("File contains {:?}", decoded),
    }
}
