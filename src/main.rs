mod metainfo;

fn main() {
    let target = "./archlinux-2020.01.01-x86_64.iso.torrent";
    let metainfo = match metainfo::read_torrent_file(target) {
        Err(error) => panic!("Errorz {:?}", error),
        Ok(decoded) => decoded,
    };
    metainfo.debug();
    let client = reqwest::blocking::Client::new();
    match client
        .get(&metainfo.announce())
        .query(&[("lang", "rust")])
        .send()
    {
        Err(e) => panic!("Error {:?}", e),
        Ok(response) => println!("{:?}", response.text()),
    };
}
