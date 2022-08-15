use std::any::Any;

use crate::ext::try_err::TryErr;
use bytes::{Buf, BufMut, BytesMut};
use std::io::{Read, Write};

/// Define the type id of the object.
pub trait TypeId: Any {
    /// Rertuns the type id.
    fn type_id(&self) -> u32 {
        Self::type_id2()
    }
    /// Rertuns the type id.
    fn type_id2() -> u32;
}

/// Serialize the data.
pub trait Serialize {
    /// Serialize the data
    fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()>;
    /// Serialize the data to bytes
    fn serialize_to_bytes(&self) -> BytesMut {
        let bytes = BytesMut::new();
        let mut writer = bytes.writer();
        self.serialize(&mut writer).unwrap();
        writer.into_inner()
    }
    /// Serialize the data to vector
    fn serialize_to_vec(&self) -> Vec<u8> {
        self.serialize_to_bytes().to_vec()
    }
}

/// Deserialize the data
pub trait Deserialize {
    /// Error type
    type Error;
    /// Deserialize the data
    fn deserialize<T: Read>(data: &mut T) -> Result<Self, Self::Error>
    where
        Self: Sized;
    /// Deserialize the data from bytes
    fn deserialize_from_bytes<T: AsRef<[u8]> + ?Sized>(data: &T) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let buf = data.as_ref();
        let mut reader = buf.reader();
        Self::deserialize(&mut reader)
    }
}

/// Deserialize the data
pub trait OptDeserialize {
    /// Error type
    type Error;
    /// Deserialize the data
    fn opt_deserialize<T: Read>(data: &mut T) -> Result<Option<Self>, Self::Error>
    where
        Self: Sized;
    /// Deserialize the data from bytes
    fn opt_deserialize_from_bytes<T: AsRef<[u8]>>(data: &T) -> Result<Option<Self>, Self::Error>
    where
        Self: Sized,
    {
        let buf = data.as_ref();
        let mut reader = buf.reader();
        Self::opt_deserialize(&mut reader)
    }
}

impl<T> Deserialize for T
where
    T: Sized + OptDeserialize,
    <T as OptDeserialize>::Error: From<&'static str>,
{
    type Error = <T as OptDeserialize>::Error;
    fn deserialize<R: Read>(data: &mut R) -> Result<Self, Self::Error> {
        Ok(T::opt_deserialize(data)?.try_err("No suitable variant found.")?)
    }
}
