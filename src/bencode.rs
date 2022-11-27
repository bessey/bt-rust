use std::collections::HashMap;

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

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    BytesValue(Vec<u8>),
    IntValue(i64),
    ListValue(List),
    DictValue(Dict),
}

const NO_REMAINDER: &[u8] = &[];

pub fn bencode_decode<'a>(raw: &'a [u8]) -> (Value, &'a [u8]) {
    match *raw.first().unwrap() as char {
        'i' => {
            let (int, remainder) = build_int(&raw);
            (Value::IntValue(int), &remainder)
        }
        '0'..='9' => {
            let (bytes, remainder) = build_bytes(&raw);
            (Value::BytesValue(bytes.to_vec()), remainder)
        }
        'l' => {
            let (list, remainder) = build_list(&raw);
            (Value::ListValue(list), remainder)
        }
        'd' => {
            let (dict, remainder) = build_dictionary(&raw);
            (Value::DictValue(dict), remainder)
        }
        x => panic!("Unexpected {:?}", x),
    }
}
fn build_int(raw: &[u8]) -> (i64, &[u8]) {
    assert!(*&raw[0] as char == 'i');

    let int_str: Vec<u8> = raw[1..]
        .into_iter()
        .map(|s| *s)
        .take_while(|x| *x as char == '-' || (*x as char >= '0' && *x as char <= '9'))
        .collect();

    assert!(*&raw[int_str.len() + 1] as char == 'e');

    let remainder = &raw[int_str.len() + 2..];
    let string = String::from_utf8_lossy(&int_str);
    return (string.parse::<i64>().unwrap(), &remainder);
}

// steps over slice, confirming each character is a 0-9 until we reach the :
// parse that into a number
// return the next <number> bytes
fn build_bytes(raw: &[u8]) -> (&[u8], &[u8]) {
    let byte_length_str: Vec<u8> = raw
        .into_iter()
        .map(|s| *s)
        .take_while(|x| *x as char >= '0' && *x as char <= '9')
        .collect();
    let byte_length: usize = String::from_utf8_lossy(&byte_length_str).parse().unwrap();
    let first_digit_idx = byte_length_str.len() + 1;
    let last_digit_idx = first_digit_idx + byte_length;
    return (
        &raw[first_digit_idx..last_digit_idx],
        &raw[last_digit_idx..],
    );
}

fn build_dictionary(raw: &[u8]) -> (Dict, &[u8]) {
    assert!(*&raw[0] as char == 'd');
    let mut dict = Dict::new();
    let mut remainder = &raw[1..];
    while *&remainder[0] as char != 'e' {
        let (key, new_remainder) = parse_dict_key(&remainder);
        let key_string = String::from_utf8(key.to_vec()).unwrap();
        let (value, new_remainder) = bencode_decode(&new_remainder);
        remainder = new_remainder;
        dict.insert(key_string, value);
    }
    return (dict, &remainder[1..]);
}

fn parse_dict_key(raw: &[u8]) -> (&[u8], &[u8]) {
    let int_str: Vec<u8> = raw
        .into_iter()
        .map(|s| *s)
        .take_while(|x| *x as char >= '0' && *x as char <= '9')
        .collect();
    assert!(*&raw[int_str.len()] as char == ':');

    let remainder = &raw[int_str.len() + 1..];
    let string = String::from_utf8_lossy(&int_str);
    let key_length = string.parse::<usize>().unwrap();
    let key = &remainder[0..key_length];
    let remainder = &remainder[key_length..];

    return (key, remainder);
}

fn build_list(raw: &[u8]) -> (List, &[u8]) {
    assert!(*&raw[0] as char == 'l');
    let mut remainder = &raw[1..];
    let list = &mut List::new();
    while *&remainder[0] as char != 'e' {
        let (list_item, new_remainder) = bencode_decode(remainder);
        list.push(list_item);
        remainder = new_remainder;
    }

    return (list.to_vec(), &remainder[1..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    const NO_REMAINDER: &[u8] = &[];

    #[test]
    fn test_build_int() {
        let raw = "i42e".as_bytes();
        assert_eq!(build_int(raw), (42, NO_REMAINDER));
    }

    #[test]
    fn test_build_int_negative() {
        let raw = "i-13e".as_bytes();
        assert_eq!(build_int(raw), (-13, NO_REMAINDER));
    }

    #[test]
    fn test_build_bytes() {
        let raw = "4:spam".as_bytes();
        assert_eq!(build_bytes(raw), ("spam".as_bytes(), NO_REMAINDER));
    }

    #[test]
    fn test_build_list() {
        let raw = "li12ei34ee".as_bytes();
        assert_eq!(
            build_list(raw),
            (
                [Value::IntValue(12), Value::IntValue(34)].to_vec(),
                NO_REMAINDER
            )
        );
    }

    // {"bar": "spam", "foo": 42}), would be encoded as follows: d3:bar4:spam3:fooi42ee
    #[test]
    fn test_build_dict() {
        let raw = "d3:bar4:spam3:fooi42ee".as_bytes();
        assert_eq!(
            build_dictionary(raw),
            (
                Dict::from([
                    (
                        "bar".to_string(),
                        Value::BytesValue("spam".as_bytes().to_vec())
                    ),
                    ("foo".to_string(), Value::IntValue(42))
                ]),
                NO_REMAINDER
            )
        );
    }
    #[test]
    fn test_bencode_decode_int() {
        let raw = "i13e".as_bytes();
        assert_eq!(bencode_decode(raw), (Value::IntValue(13), NO_REMAINDER));
    }

    #[test]
    fn test_bencode_decode_bytes() {
        let raw = "4:spam".as_bytes();
        assert_eq!(
            bencode_decode(raw),
            (Value::BytesValue("spam".as_bytes().to_vec()), NO_REMAINDER)
        );
    }
}
