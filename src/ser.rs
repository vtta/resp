use std::{fmt, str};
use std::io::Write;

use serde::{ser, Serialize};

use crate::{Error, Result};

/// Serializer
pub struct Serializer {
    /// This string starts empty and RESP is appended as values are serialized.
    output: Vec<u8>,
}

/// serialize to string
pub fn to_string<T>(value: &T) -> Result<String>
    where
        T: Serialize,
{
    let mut serializer = Serializer { output: Vec::new() };
    value.serialize(&mut serializer)?;
    Ok(String::from_utf8(serializer.output)?)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    // The output type produced by this `Serializer` during successful
    // serialization. Most serializers that produce text or binary output should
    // set `Ok = ()` and serialize into an `io::Write` or buffer contained
    // within the `Serializer` instance, as happens here. Serializers that build
    // in-memory data structures may be simplified by using `Ok` to propagate
    // the data structure around.
    type Ok = ();
    type Error = Error;

    // Associated types for keeping track of additional state while serializing
    // compound data structures like sequences and maps. In this case no
    // additional state is required beyond what is already stored in the
    // Serializer struct.
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    /// 0 for false other for true
    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        self.serialize_i64(v as i64)
    }
    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        self.serialize_i64(v as i64)
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        self.serialize_i64(v as i64)
    }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        self.output.write_all(b":")?;
        self.output.write_all(&v.to_string().as_bytes())?;
        self.output.write_all(b"\r\n")?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        self.serialize_u64(v as u64)
    }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        self.serialize_u64(v as u64)
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        self.output.write_all(b":")?;
        self.output.write_all(&v.to_string().as_bytes())?;
        self.output.write_all(b"\r\n")?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        self.serialize_f64(v as f64)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        if v.as_bytes().contains(&b'\r') || v.as_bytes().contains(&b'\n') {
            self.serialize_bytes(v.as_bytes())?;
        } else {
            self.output.write_all(b"+")?;
            self.output.write_all(&v.as_bytes())?;
            self.output.write_all(b"\r\n")?;
        }
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        self.output.write_all(b"$")?;
        self.output.write_all(&v.len().to_string().as_bytes())?;
        self.output.write_all(b"\r\n")?;
        self.output.write_all(v)?;
        self.output.write_all(b"\r\n")?;
        Ok(())
    }

    // use an empty array to represent None
    // None => []
    fn serialize_none(self) -> Result<Self::Ok> {
        Vec::<String>::new().serialize(self)
    }

    // use an array with a single object as Some
    // Some(value) => [value]
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok>
        where
            T: Serialize,
    {
        vec![value].serialize(self)
    }

    // use null as unit which is "$-1\r\n"
    fn serialize_unit(self) -> Result<Self::Ok> {
        self.output.write_all(b"$-1\r\n")?;
        Ok(())
    }

    // struct Unit or PhantomData<T>
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    // E::A and E::B in enum E { A, B }
    // use the variant name directly
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.serialize_str(variant)
    }

    // struct Millimeters(u8)
    // serializers are encouraged to treat newtype structs as
    // insignificant wrappers around the data they contain
    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
        where
            T: Serialize,
    {
        value.serialize(self)
    }

    // E::N in enum E { N(u8) }
    // treat it as [NAME, VALUE]
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
        where
            T: Serialize,
    {
        self.output.write_all(b"*2\r\n")?;
        variant.serialize(&mut *self)?;
        value.serialize(&mut *self)?;
        Ok(())
    }

    // The start of the sequence, each value, and the end are three separate
    // method calls. This one is responsible only for serializing the start
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        let len = len.ok_or(Error::LenNotKnown)?;
        self.output.write_all(b"*")?;
        self.output.write_all(&len.to_string().as_bytes())?;
        self.output.write_all(b"\r\n")?;
        Ok(self)
    }

    // [value0, value1, ...]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    // [field0, field1, ...]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    // [variant, [field0, field1, ...]]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.output.write_all(b"*2\r\n")?;
        variant.serialize(&mut *self)?;
        self.serialize_seq(Some(len))
    }

    //  [[k0,v0], [k1,v1], ...]
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        let len = len.ok_or(Error::LenNotKnown)?;
        self.output.write_all(b"*")?;
        self.output.write_all(len.to_string().as_bytes())?;
        self.output.write_all(b"\r\n")?;
        Ok(self)
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    // [variant, [[k0,v0], [k1,v1], ...]]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.output.write_all(b"*2\r\n")?;
        variant.serialize(&mut *self)?;
        self.serialize_seq(Some(len))
    }

    fn collect_str<T: ?Sized>(self, value: &T) -> Result<Self::Ok>
        where
            T: fmt::Display,
    {
        self.serialize_str(&format!("{}", value))
    }

    fn is_human_readable(&self) -> bool {
        true
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
        where
            T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
        where
            T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
        where
            T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
        where
            T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<()>
        where
            T: Serialize,
    {
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<()>
        where
            T: Serialize,
    {
        Ok(())
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(&mut self, key: &K, value: &V) -> Result<()>
        where
            K: Serialize,
            V: Serialize,
    {
        self.output.write_all(b"*2\r\n")?;
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)?;

        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
        where
            T: Serialize,
    {
        self.output.write_all(b"*2\r\n")?;
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
        where
            T: Serialize,
    {
        self.output.write_all(b"*2\r\n")?;
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct() {
        #[derive(Serialize)]
        struct Test {
            int: u32,
            seq: Vec<&'static str>,
        }

        let test = Test {
            int: 1,
            seq: vec!["a", "b"],
        };
        let expected = "*2\r\n*2\r\n+int\r\n:1\r\n*2\r\n+seq\r\n*2\r\n+a\r\n+b\r\n";
        // [[int, 1], [seq, [a, b]]
        // *2\r\n
        //   *2\r\n
        //     +int\r\n
        //     :1\r\n
        //   *2\r\n
        //     +seq\r\n
        //     *2\r\n
        //       +a\r\n
        //       +b\r\n
        assert_eq!(to_string(&test).unwrap(), expected);
    }

    #[test]
    fn test_enum() {
        #[derive(Serialize)]
        enum E {
            Unit,
            Newtype(u32),
            Tuple(u32, u32),
            Struct { a: u32 },
        }

        let u = E::Unit;
        // Unit
        let expected = "+Unit\r\n";
        assert_eq!(to_string(&u).unwrap(), expected);

        let n = E::Newtype(1);
        // [Newtype, 1]
        let expected = "*2\r\n+Newtype\r\n:1\r\n";
        //assert_eq!(to_string(&n).unwrap(), expected);

        let t = E::Tuple(1, 2);
        // [Tuple, [1, 2]]
        let expected = "*2\r\n+Tuple\r\n*2\r\n:1\r\n:2\r\n";
        assert_eq!(to_string(&t).unwrap(), expected);

        let s = E::Struct { a: 1 };
        // [Struct, [[a, 1]]]
        // *2\r\n
        //   +Struct\r\n
        //   *1\r\n
        //     *2\r\n
        //       +a\r\n
        //       :1\r\n
        let expected = "*2\r\n+Struct\r\n*1\r\n*2\r\n+a\r\n:1\r\n";
        assert_eq!(to_string(&s).unwrap(), expected);
    }
}
