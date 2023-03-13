use std::ops::{AddAssign, MulAssign, Neg};
use std::str::Bytes;

use serde::de::{
    self, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess, VariantAccess,
    Visitor,
};
use serde::Deserialize;

use error::{Error, Result};

use super::error;

pub struct Deserializer<'de> {
    // This string starts with the input data and characters are truncated off
    // the beginning as data is parsed.
    input: &'de [u8],
}

pub fn from_bytes<'a, T>(s: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_bytes(s);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(Error::TrailingCharacters)
    }
}
impl<'de> Deserializer<'de> {
    pub fn from_bytes(input: &'de [u8]) -> Self {
        Deserializer { input }
    }

    // Look at the first character in the input without consuming it.
    fn peek_char(&mut self) -> Result<char> {
        match self.input.first() {
            Some(byte) => Ok(*byte as char),
            None => Err(Error::Eof),
        }
    }

    // Consume the first character in the input.
    fn next_char(&mut self) -> Result<char> {
        let ch = self.peek_char()?;
        self.input = &self.input[1..];
        Ok(ch)
    }

    // Parse a group of decimal digits as a signed integer of type T.
    fn parse_signed<T>(&mut self) -> Result<T>
    where
        T: AddAssign<T> + MulAssign<T> + Neg<Output = T> + From<u8>,
    {
        if self.next_char()? != 'i' {
            return Err(Error::Syntax);
        };

        let mut negative = false;
        if self.peek_char()? == '-' {
            negative = true;
            self.next_char()?;
        }

        let mut int = match self.next_char()? {
            ch @ '0'..='9' => T::from(ch as u8 - b'0'),
            _ => {
                return Err(Error::ExpectedInteger);
            }
        };
        loop {
            match self.next_char()? {
                ch @ '0'..='9' => {
                    int *= T::from(10);
                    int += T::from(ch as u8 - b'0');
                }
                'e' => {
                    if negative {
                        return Ok(int.neg());
                    } else {
                        return Ok(int);
                    }
                }
                _ => return Err(Error::ExpectedIntegerOrEnd),
            }
        }
    }

    fn parse_bytes(&mut self) -> Result<&'de [u8]> {
        let mut length = match self.next_char()? {
            ch @ '0'..='9' => usize::from(ch as u8 - b'0'),
            _ => {
                return Err(Error::ExpectedInteger);
            }
        };
        loop {
            match self.next_char()? {
                ch @ '0'..='9' => {
                    length *= 10;
                    length += usize::from(ch as u8 - b'0');
                }
                ':' => break,
                _ => return Err(Error::ExpectedIntegerOrEnd),
            }
        }
        let bytes = &self.input[..length];
        self.input = &self.input[length..];
        return Ok(bytes);
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    // Look at the input data to decide what Serde data model type to
    // deserialize as. Not all data formats are able to support this operation.
    // Formats that support `deserialize_any` are known as self-describing.
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.peek_char()? {
            'i' => self.deserialize_i64(visitor),
            '0'..='9' => self.deserialize_bytes(visitor),
            'l' => self.deserialize_seq(visitor),
            'd' => self.deserialize_map(visitor),
            _ => Err(Error::Syntax),
        }
    }

    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.parse_signed()?)
    }
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.parse_signed()?)
    }
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.parse_signed()?)
    }
    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_bytes(self.parse_bytes()?)
    }
    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Parse the opening bracket of the sequence.
        if !(self.next_char()? == 'l') {
            return Err(Error::Syntax);
        }
        // Give the visitor access to each element of the sequence.
        let value = visitor.visit_seq(ListWrapped::new(self))?;
        // Parse the closing bracket of the sequence.
        if self.next_char()? == 'e' {
            Ok(value)
        } else {
            Err(Error::ExpectedEnd)
        }
    }
    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Parse the opening brace of the map.
        if self.next_char()? != 'd' {
            return Err(Error::Syntax);
        }

        // Give the visitor access to each entry of the map.
        let value = visitor.visit_map(ListWrapped::new(self))?;
        // Parse the closing brace of the map.
        if self.next_char()? == 'e' {
            Ok(value)
        } else {
            Err(Error::ExpectedEnd)
        }
    }
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }
    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }
    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}

struct ListWrapped<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> ListWrapped<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        ListWrapped { de }
    }
}

// `SeqAccess` is provided to the `Visitor` to give it the ability to iterate
// through elements of the sequence.
impl<'de, 'a> SeqAccess<'de> for ListWrapped<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        // Check if there are no more elements.
        if self.de.peek_char()? == 'e' {
            return Ok(None);
        }
        // Deserialize an array element.
        seed.deserialize(&mut *self.de).map(Some)
    }
}

// `MapAccess` is provided to the `Visitor` to give it the ability to iterate
// through entries of the map.
impl<'de, 'a> MapAccess<'de> for ListWrapped<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        // Check if there are no more entries.
        if self.de.peek_char()? == 'e' {
            return Ok(None);
        }
        // Deserialize a map key.
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        // Deserialize a map value.
        seed.deserialize(&mut *self.de)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int() {
        assert_eq!(123456, from_bytes::<i32>(b"i123456e").unwrap());
        assert_eq!(1234, from_bytes::<i32>(b"i01234e").unwrap());
        assert_eq!(-321, from_bytes::<i32>(b"i-321e").unwrap());
    }

    #[test]
    fn test_bytes() {
        assert_eq!(b"spam", from_bytes::<&[u8]>(b"4:spam").unwrap());
    }

    #[test]
    fn test_homogenous_list() {
        assert_eq!(
            vec![123, 321],
            from_bytes::<Vec<i32>>(b"li123ei321ee").unwrap()
        );
        assert_eq!(
            vec![b"foo", b"bar"],
            from_bytes::<Vec<&[u8]>>(b"l3:foo3:bare").unwrap()
        );
    }

    #[test]
    fn test_map() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct S<'a> {
            bar: &'a [u8],
            foo: i32,
        }
        assert_eq!(
            S {
                bar: b"spam",
                foo: 42
            },
            from_bytes::<S>(b"d3:bar4:spam3:fooi42ee").unwrap()
        );
    }

    #[test]
    fn test_the_lot() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct S<'a> {
            bar: &'a [u8],
            foo: Vec<i32>,
            #[serde(borrow)]
            baz: Vec<&'a [u8]>,
            zap: R<'a>,
        }

        #[derive(Deserialize, PartialEq, Debug)]
        struct R<'b> {
            #[serde(borrow)]
            taz: Vec<&'b [u8]>,
        }
        assert_eq!(
            S {
                bar: b"spam",
                foo: vec![12, 34],
                baz: vec![b"foo"],
                zap: R { taz: vec![b"bar"] }
            },
            from_bytes::<S>(b"d3:bar4:spam3:fooli12ei34ee3:bazl3:fooe3:zapd3:tazl3:bareee")
                .unwrap()
        );
    }
}
