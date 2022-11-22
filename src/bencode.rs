// #[derive(Debug, Deserialize)]
// pub struct Node(String, i64);

// #[derive(Debug, Deserialize)]
// pub struct File {
//     path: Vec<String>,
//     length: i64,
//     #[serde(default)]
//     md5sum: Option<String>,
// }

use std::collections::HashMap;

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

    let result = bencode_decode(metainfo);
    println!("Value: {:?}", result);

    return Torrent { info: info };
}

// From Wikipedia:
//
// An integer is encoded as i<integer encoded in base ten ASCII>e. Leading zeros are not allowed (although the number
// zero is still represented as "0"). Negative values are encoded by prefixing the number with a hyphen-minus. The
// number 42 would thus be encoded as i42e, 0 as i0e, and -42 as i-42e. Negative zero is not permitted.
//
// A byte string (a sequence of bytes, not necessarily characters) is encoded as <length>:<contents>. The length is
// encoded in base 10, like integers, but must be non-negative (zero is allowed); the contents are just the bytes that
// make up the string. The string "spam" would be encoded as 4:spam. The specification does not deal with encoding of
// characters outside the ASCII set; to mitigate this, some BitTorrent applications explicitly communicate the encoding
// (most commonly UTF-8) in various non-standard ways. This is identical to how netstrings work, except that netstrings
// additionally append a comma suffix after the byte sequence.
//
// A list of values is encoded as l<contents>e . The contents consist of the bencoded elements of the list, in order,
// concatenated. A list consisting of the string "spam" and the number 42 would be encoded as: l4:spami42ee. Note the
// absence of separators between elements, and the first character is the letter 'l', not digit '1'.
//
// A dictionary is encoded as d<contents>e. The elements of the dictionary are encoded with each key immediately
// followed by its value. All keys must be byte strings and must appear in lexicographical order. A dictionary that
// associates the values 42 and "spam" with the keys "foo" and "bar", respectively (in other words,
// {"bar": "spam", "foo": 42}), would be encoded as follows: d3:bar4:spam3:fooi42ee.

type List = Vec<Value>;
type Dict = HashMap<String, Value>;

#[derive(Debug, PartialEq)]
pub enum Value {
    BytesValue(Vec<u8>),
    IntValue(i64),
    ListValue(List),
    DictValue(Dict),
}
pub fn bencode_decode(raw: Vec<u8>) -> Value {
    let start = *raw.first().unwrap() as char;
    let end = *raw.last().unwrap() as char;
    let last = raw.len() - 1;
    let contents = &raw[1..last];
    match (start, end) {
        ('i', 'e') => Value::IntValue(build_int(contents)),
        ('l', 'e') => Value::ListValue(build_list(contents)),
        ('d', 'e') => Value::DictValue(build_dictionary(contents)),
        // Bytes are unusual in not terminating with an e
        ('0'..='9', ..) => Value::BytesValue(build_bytes(&raw).to_vec()),
        _ => panic!("Invalid"),
    }
}
fn build_int(raw: &[u8]) -> i64 {
    let string = String::from_utf8_lossy(raw);
    return string.parse::<i64>().unwrap();
}

fn build_bytes(raw: &[u8]) -> &[u8] {
    // step over slice, confirming each character is a 0-9 until we reach the :
    // parse that into a number
    // return the next <number> bytes

    let byte_length_str: Vec<u8> = raw
        .into_iter()
        .map(|s| *s)
        .take_while(|x| *x as char >= '0' && *x as char <= '9')
        .collect();
    let byte_length: usize = String::from_utf8_lossy(&byte_length_str).parse().unwrap();
    let first_digit_idx = byte_length_str.len() + 1;
    let last_digit_idx = first_digit_idx + byte_length;
    return &raw[first_digit_idx..last_digit_idx];
}

fn build_dictionary(raw: &[u8]) -> Dict {
    return Dict::new();
}

fn build_list(raw: &[u8]) -> List {
    return List::new();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_int() {
        let raw = "42".as_bytes();
        assert_eq!(build_int(raw), 42);
    }

    #[test]
    fn test_build_int_negative() {
        let negative = "-13".as_bytes();
        assert_eq!(build_int(negative), -13);
    }

    #[test]
    fn test_build_bytes() {
        let raw = "4:spam".as_bytes();
        assert_eq!(build_bytes(raw), "spam".as_bytes());
    }

    #[test]
    fn test_bencode_decode_int() {
        let raw = "i13e".as_bytes().to_vec();
        assert_eq!(bencode_decode(raw), Value::IntValue(13));
    }

    #[test]
    fn test_bencode_decode_bytes() {
        let raw = "4:spam".as_bytes().to_vec();
        assert_eq!(
            bencode_decode(raw),
            Value::BytesValue("spam".as_bytes().to_vec())
        );
    }
}
