use core::str;
use std::fmt;

use serde::de::{self, IgnoredAny, Visitor};

pub fn from_bytes<'de, T: de::Deserialize<'de>>(
    data: &'de [u8],
    with_event: bool,
) -> Result<T, rmp_serde::decode::Error> {
    let inner = &mut rmp_serde::Deserializer::new(data);
    let de = Deserializer {
        inner,
        skip_first_element: with_event,
    };
    T::deserialize(de)
}

pub fn from_bytes_seed<'de, T: de::DeserializeSeed<'de>>(
    data: &'de [u8],
    seed: T,
    with_event: bool,
) -> Result<T::Value, rmp_serde::decode::Error> {
    let inner = &mut rmp_serde::Deserializer::new(data);
    let de = Deserializer {
        inner,
        skip_first_element: with_event,
    };
    seed.deserialize(de)
}

/// It will read the first element of the array as the socket.io event.
/// The event must be a string.
pub fn read_event(data: &[u8]) -> Result<&str, rmp_serde::decode::Error> {
    let rd = &mut &data[..];
    let arrlen = rmp::decode::read_array_len(rd)?;
    if arrlen < 1 {
        return Err(rmp_serde::decode::Error::OutOfRange);
    }
    let strlen = rmp::decode::read_str_len(rd)? as usize;
    str::from_utf8(&rd[..strlen]).map_err(rmp_serde::decode::Error::Utf8Error)
}

struct Deserializer<D> {
    inner: D,
    skip_first_element: bool,
}

impl<'de, D: de::Deserializer<'de>> de::Deserializer<'de> for Deserializer<D> {
    type Error = D::Error;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.inner.deserialize_any(visitor)
    }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.inner.deserialize_bool(visitor)
    }

    fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.inner.deserialize_i8(visitor)
    }

    fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.inner.deserialize_i16(visitor)
    }

    fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.inner.deserialize_i32(visitor)
    }

    fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.inner.deserialize_i64(visitor)
    }

    fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.inner.deserialize_u8(visitor)
    }

    fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.inner.deserialize_u16(visitor)
    }

    fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.inner.deserialize_u32(visitor)
    }

    fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.inner.deserialize_u64(visitor)
    }

    fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.inner.deserialize_f32(visitor)
    }

    fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.inner.deserialize_f64(visitor)
    }

    fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.inner.deserialize_char(visitor)
    }

    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.inner.deserialize_str(visitor)
    }

    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.inner.deserialize_string(visitor)
    }

    fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.inner.deserialize_bytes(visitor)
    }

    fn deserialize_byte_buf<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.inner.deserialize_byte_buf(visitor)
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.inner.deserialize_option(visitor)
    }

    fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.inner.deserialize_unit(visitor)
    }

    fn deserialize_unit_struct<V: Visitor<'de>>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.inner.deserialize_unit_struct(name, visitor)
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.inner.deserialize_newtype_struct(name, visitor)
    }

    fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.inner
            .deserialize_seq(SeqVisitor::new(visitor, self.skip_first_element))
    }

    fn deserialize_tuple<V: Visitor<'de>>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.inner
            .deserialize_tuple(len, SeqVisitor::new(visitor, self.skip_first_element))
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.inner.deserialize_tuple_struct(
            name,
            len,
            SeqVisitor::new(visitor, self.skip_first_element),
        )
    }

    fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.inner.deserialize_map(visitor)
    }

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.inner.deserialize_struct(name, fields, visitor)
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.inner.deserialize_enum(name, variants, visitor)
    }

    fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.inner.deserialize_identifier(visitor)
    }

    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.inner.deserialize_ignored_any(visitor)
    }
}

struct SeqVisitor<V> {
    inner: V,
    skip_first_element: bool,
}
impl<V> SeqVisitor<V> {
    fn new(inner: V, skip_first_element: bool) -> Self {
        Self {
            inner,
            skip_first_element,
        }
    }
}

impl<'de, V: Visitor<'de>> Visitor<'de> for SeqVisitor<V> {
    type Value = V::Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a sequence")
    }
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        if self.skip_first_element {
            let _ = seq.next_element::<IgnoredAny>()?; // We ignore the event value
        }
        self.inner.visit_seq(seq)
    }
}
