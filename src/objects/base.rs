use super::error::DeserializeError;
use super::traits::{Deserialize, OptDeserialize, Serialize, TypeId};
use bytes::BytesMut;
use std::ffi::{CStr, CString};
use std::io::Read;

impl<T> Serialize for Box<T>
where
    T: Serialize + TypeId,
{
    fn serialize(&self) -> Vec<u8> {
        let mut v = Vec::with_capacity(4);
        v.extend_from_slice(&TypeId::type_id(self.as_ref()).to_le_bytes());
        v.extend_from_slice(&self.as_ref().serialize());
        v
    }
}

impl<T> OptDeserialize for Box<T>
where
    T: Deserialize<Error = DeserializeError> + TypeId + Sized,
{
    type Error = DeserializeError;
    fn opt_deserialize<R: Read>(data: &mut R) -> Result<Option<Self>, Self::Error> {
        let type_id = u32::deserialize(data)?;
        if type_id != T::type_id2() {
            return Ok(None);
        }
        Ok(Some(Box::new(T::deserialize(data)?)))
    }
}

impl<T> TypeId for Vec<T>
where
    T: 'static,
{
    fn type_id2() -> u32 {
        0x1cb5c415
    }
}

impl<T> Serialize for Vec<T>
where
    T: Serialize + 'static,
{
    fn serialize(&self) -> Vec<u8> {
        let mut v = Vec::with_capacity(4);
        v.extend_from_slice(&(self.len() as u32).to_le_bytes());
        for i in self.iter() {
            v.extend_from_slice(&i.serialize());
        }
        v
    }
}

impl<T> Deserialize for Vec<T>
where
    T: Deserialize<Error = DeserializeError> + Sized,
{
    type Error = DeserializeError;
    fn deserialize<R: Read>(data: &mut R) -> Result<Self, Self::Error> {
        let le = u32::deserialize(data)?;
        let mut v = Vec::with_capacity(le as usize);
        for _ in 0..le - 1 {
            v.push(T::deserialize(data)?);
        }
        Ok(v)
    }
}

impl TypeId for i32 {
    fn type_id2() -> u32 {
        0xa8509bda // int ? = Int
    }
}

impl Serialize for i32 {
    fn serialize(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Deserialize for i32 {
    type Error = DeserializeError;
    fn deserialize<R: Read>(data: &mut R) -> Result<Self, Self::Error> {
        let mut buf = [0u8; 4];
        data.read_exact(&mut buf)?;
        Ok(i32::from_le_bytes(buf))
    }
}

impl Deserialize for u32 {
    type Error = DeserializeError;
    fn deserialize<R: Read>(data: &mut R) -> Result<Self, Self::Error> {
        let mut buf = [0u8; 4];
        data.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }
}

impl TypeId for i64 {
    fn type_id2() -> u32 {
        0x22076cba // long ? = Long
    }
}

impl Serialize for i64 {
    fn serialize(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Deserialize for i64 {
    type Error = DeserializeError;
    fn deserialize<R: Read>(data: &mut R) -> Result<Self, Self::Error> {
        let mut buf = [0u8; 8];
        data.read_exact(&mut buf)?;
        Ok(i64::from_le_bytes(buf))
    }
}

impl TypeId for f64 {
    fn type_id2() -> u32 {
        0x2210c154 // double ? = Double
    }
}

impl Serialize for f64 {
    fn serialize(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Deserialize for f64 {
    type Error = DeserializeError;
    fn deserialize<R: Read>(data: &mut R) -> Result<Self, Self::Error> {
        let mut buf = [0u8; 8];
        data.read_exact(&mut buf)?;
        Ok(f64::from_le_bytes(buf))
    }
}

impl TypeId for String {
    fn type_id2() -> u32 {
        0xb5286e24 // string ? = String
    }
}

impl TypeId for CString {
    fn type_id2() -> u32 {
        0xb5286e24 // string ? = String
    }
}

impl Serialize for String {
    fn serialize(&self) -> Vec<u8> {
        self.as_str().serialize()
    }
}

impl Deserialize for String {
    type Error = DeserializeError;
    fn deserialize<R: Read>(data: &mut R) -> Result<Self, Self::Error> {
        let s = CString::deserialize(data)?;
        Ok(s.to_str()?.to_owned())
    }
}

impl Serialize for CString {
    fn serialize(&self) -> Vec<u8> {
        self.as_c_str().serialize()
    }
}

impl Deserialize for CString {
    type Error = DeserializeError;
    fn deserialize<R: Read>(data: &mut R) -> Result<Self, Self::Error> {
        let s = BytesMut::deserialize(data)?;
        Ok(CString::new(s.as_ref())?)
    }
}

impl Deserialize for BytesMut {
    type Error = DeserializeError;
    fn deserialize<R: Read>(data: &mut R) -> Result<Self, Self::Error> {
        let mut le = [0u8; 1];
        data.read_exact(&mut le)?;
        let (le, pd) = if le[0] == 255 {
            return Err(DeserializeError::from("The data is not a string."));
        } else if le[0] != 254 {
            (le[0] as u32, (3 - le[0] % 4) as u32)
        } else {
            let mut le2 = [0u8; 3];
            data.read_exact(&mut le2)?;
            let le2 = [le2[0], le2[1], le2[2], 0];
            let le = u32::from_le_bytes(le2);
            (le, 3 - ((le - 1) % 4))
        };
        let mut s = BytesMut::with_capacity(le as usize);
        s.resize(le as usize, 0);
        data.read_exact(&mut s)?;
        if pd > 0 {
            let mut pdb = BytesMut::with_capacity(pd as usize);
            pdb.resize(pd as usize, 0);
            data.read_exact(&mut pdb)?;
        }
        Ok(s)
    }
}

impl TypeId for str {
    fn type_id2() -> u32 {
        0xb5286e24 // string ? = String
    }
}

impl TypeId for CStr {
    fn type_id2() -> u32 {
        0xb5286e24 // string ? = String
    }
}

impl Serialize for str {
    fn serialize(&self) -> Vec<u8> {
        let mut v = Vec::with_capacity(4);
        let le = self.len() as u32;
        if le <= 253 {
            v.push(le as u8);
            v.extend_from_slice(self.as_bytes());
            let m = 3 - (le % 4);
            for _ in 0..m {
                v.push(0);
            }
        } else {
            v.push(254);
            v.extend_from_slice(&le.to_le_bytes()[..3]);
            v.extend_from_slice(self.as_bytes());
            let m = 3 - ((le - 1) % 4);
            for _ in 0..m {
                v.push(0);
            }
        }
        v
    }
}

impl Serialize for CStr {
    fn serialize(&self) -> Vec<u8> {
        let mut v = Vec::with_capacity(4);
        let byte = self.to_bytes();
        let le = byte.len() as u32;
        if le <= 253 {
            v.push(le as u8);
            v.extend_from_slice(byte);
            let m = 3 - (le % 4);
            for _ in 0..m {
                v.push(0);
            }
        } else {
            v.push(254);
            v.extend_from_slice(&le.to_le_bytes()[..3]);
            v.extend_from_slice(byte);
            let m = 3 - ((le - 1) % 4);
            for _ in 0..m {
                v.push(0);
            }
        }
        v
    }
}

impl TypeId for i128 {
    fn type_id2() -> u32 {
        0x84ccf7b7 // int128 4*[ int ] = Int128
    }
}

impl Serialize for i128 {
    fn serialize(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Deserialize for i128 {
    type Error = DeserializeError;
    fn deserialize<R: Read>(data: &mut R) -> Result<Self, Self::Error> {
        let mut buf = [0u8; 16];
        data.read_exact(&mut buf)?;
        Ok(i128::from_le_bytes(buf))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// int256
pub struct I256 {
    data: [u64; 4],
}

impl From<u128> for I256 {
    fn from(v: u128) -> Self {
        Self {
            data: [0, 0, (v >> 64) as u64, v as u64],
        }
    }
}

impl TypeId for I256 {
    fn type_id2() -> u32 {
        0x9fcb633e
    }
}

impl Serialize for I256 {
    fn serialize(&self) -> Vec<u8> {
        let mut v = Vec::with_capacity(32);
        v.extend_from_slice(&self.data[3].to_le_bytes());
        v.extend_from_slice(&self.data[2].to_le_bytes());
        v.extend_from_slice(&self.data[1].to_le_bytes());
        v.extend_from_slice(&self.data[0].to_le_bytes());
        v
    }
}

impl Deserialize for I256 {
    type Error = DeserializeError;
    fn deserialize<R: Read>(data: &mut R) -> Result<Self, Self::Error> {
        let mut buf = [0u8; 32];
        data.read_exact(&mut buf)?;
        Ok(Self {
            data: [
                u64::from_le_bytes(buf[24..32].try_into().unwrap()),
                u64::from_le_bytes(buf[16..24].try_into().unwrap()),
                u64::from_le_bytes(buf[8..16].try_into().unwrap()),
                u64::from_le_bytes(buf[0..8].try_into().unwrap()),
            ],
        })
    }
}

/// Message struct
#[derive(Clone, Debug)]
pub struct Message {
    /// The auth key
    pub auth_key_id: i64,
    /// Message id
    pub message_id: i64,
    /// Payload
    pub payload: BytesMut,
}

impl Message {
    /// Deserialize the message payload
    pub fn deserialize_payload<T: Deserialize>(&self) -> Result<T, T::Error> {
        T::deserialize_from_bytes(&self.payload)
    }
    /// Deserialize the message payload
    pub fn opt_deserialize_payload<T: OptDeserialize>(&self) -> Result<Option<T>, T::Error> {
        T::opt_deserialize_from_bytes(&self.payload)
    }
}

impl Deserialize for Message {
    type Error = DeserializeError;
    fn deserialize<R: Read>(data: &mut R) -> Result<Self, Self::Error> {
        let auth_key_id = i64::deserialize(data)?;
        let message_id = i64::deserialize(data)?;
        let message_len = u32::deserialize(data)?;
        let mut payload = BytesMut::with_capacity(message_len as usize);
        payload.resize(message_len as usize, 0);
        data.read_exact(&mut payload)?;
        Ok(Self {
            auth_key_id,
            message_id,
            payload,
        })
    }
}

#[test]
fn test_type_id() {
    assert_eq!(vec![1].type_id(), 0x1cb5c415);
    assert_eq!(3.type_id(), 0xa8509bda);
}

#[test]
fn test_serialize() {
    assert_eq!((-1).serialize(), vec![0xff, 0xff, 0xff, 0xff]);
    assert_eq!(3223235.serialize(), vec![0xc3, 0x2e, 0x31, 0x00]);
    assert_eq!(
        Box::new(vec![1, 2, 3]).serialize(),
        vec![0x15, 0xc4, 0xb5, 0x1c, 3, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0]
    );
    assert_eq!(1i64.serialize(), vec![0x1, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(
        0x23456789abi64.serialize(),
        vec![0xab, 0x89, 0x67, 0x45, 0x23, 0, 0, 0]
    );
    assert_eq!(
        3.2.serialize(),
        vec![0x9a, 0x99, 0x99, 0x99, 0x99, 0x99, 0x9, 0x40]
    );
    assert_eq!("".serialize(), vec![0, 0, 0, 0]);
    assert_eq!(
        "hello".serialize(),
        vec![5, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0, 0]
    );
    let s = String::from("s").repeat(256);
    let mut v = vec![0xfe, 0, 1, 0];
    v.extend_from_slice(s.as_bytes());
    assert_eq!(s.serialize(), v);
    let i = I256::from(1);
    let mut v = vec![1];
    v.extend_from_slice(&[0; 31]);
    assert_eq!(i.serialize(), v);
    let mut v = vec![1];
    v.extend_from_slice(&[0; 15]);
    assert_eq!(1i128.serialize(), v);
    let s = CString::new(vec![1u8, 2, 3]).unwrap();
    assert_eq!(s.as_bytes().len(), 3);
    assert_eq!(s.serialize(), vec![3, 1, 2, 3]);
}

#[test]
fn test_deserialize() {
    assert_eq!(i32::deserialize_from_bytes(&(-1).serialize()).unwrap(), -1);
    assert_eq!(
        u32::deserialize_from_bytes(&(-1).serialize()).unwrap(),
        0xffffffff
    );
    assert_eq!(
        i64::deserialize_from_bytes(&(2313213i64).serialize()).unwrap(),
        2313213
    );
    assert_eq!(
        f64::deserialize_from_bytes(&(23232f64).serialize()).unwrap(),
        23232f64
    );
    assert_eq!(
        String::deserialize_from_bytes(&("hello".serialize())).unwrap(),
        String::from("hello")
    );
    let s = String::from("s").repeat(256);
    assert_eq!(String::deserialize_from_bytes(&s.serialize()).unwrap(), s);
    let cs = CString::new("s2d").unwrap();
    assert_eq!(
        CString::deserialize_from_bytes(&cs.serialize()).unwrap(),
        cs
    );
    assert_eq!(
        i128::deserialize_from_bytes(&(2313213239210938210391283i128).serialize()).unwrap(),
        2313213239210938210391283
    );
    let i = I256 { data: [1, 2, 3, 4] };
    assert_eq!(I256::deserialize_from_bytes(&i.serialize()).unwrap(), i);
}
