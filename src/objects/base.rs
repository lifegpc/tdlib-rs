use super::traits::{Serialize, TypeId};
use std::ffi::{CStr, CString};

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

impl<T> TypeId for Vec<T>
where
    T: 'static,
{
    fn type_id(&self) -> u32 {
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

impl TypeId for i32 {
    fn type_id(&self) -> u32 {
        0xa8509bda // int ? = Int
    }
}

impl Serialize for i32 {
    fn serialize(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl TypeId for i64 {
    fn type_id(&self) -> u32 {
        0x22076cba // long ? = Long
    }
}

impl Serialize for i64 {
    fn serialize(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl TypeId for f64 {
    fn type_id(&self) -> u32 {
        0x2210c154 // double ? = Double
    }
}

impl Serialize for f64 {
    fn serialize(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl TypeId for String {
    fn type_id(&self) -> u32 {
        0xb5286e24 // string ? = String
    }
}

impl TypeId for CString {
    fn type_id(&self) -> u32 {
        0xb5286e24 // string ? = String
    }
}

impl Serialize for String {
    fn serialize(&self) -> Vec<u8> {
        self.as_str().serialize()
    }
}

impl Serialize for CString {
    fn serialize(&self) -> Vec<u8> {
        self.as_c_str().serialize()
    }
}

impl TypeId for str {
    fn type_id(&self) -> u32 {
        0xb5286e24 // string ? = String
    }
}

impl TypeId for CStr {
    fn type_id(&self) -> u32 {
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
    fn type_id(&self) -> u32 {
        0x84ccf7b7 // int128 4*[ int ] = Int128
    }
}

impl Serialize for i128 {
    fn serialize(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

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
    fn type_id(&self) -> u32 {
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
