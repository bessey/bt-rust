use std::fs::File;

fn main() {
    println!("Hello, world!");
    let target = "./archlinux-2020.01.01-x86_64.iso.torrent";
    let file = File::open(target);
    match file {
        Ok(contents) => println!("File contains {:?}", contents),
        Err(e) => println!("Error {:?}", e),
    }
}
