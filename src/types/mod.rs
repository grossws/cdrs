/// Cassandra types

pub const LONG_STR_LEN: usize = 4;
pub const SHORT_LEN: usize = 2;
pub const INT_LEN: usize = 4;
pub const UUID_LEN: usize = 16;

use std::io;
use std::io::{Cursor, Read};
use std::net::SocketAddr;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt, ByteOrder};
use {FromBytes, IntoBytes, FromCursor};
use error::Result as CDRSResult;
use types::data_serialization_types::decode_inet;

pub mod data_serialization_types;
pub mod list;
pub mod map;
pub mod rows;
pub mod udt;
pub mod value;

/// Should be used to represent a single column as a Rust value.
// TODO: change Option to Result, create a new type of error for that.
pub trait AsRust<T> {
    fn as_rust(&self) -> CDRSResult<T>;
}

/// Should be used to return a single column as Rust value by its name.
pub trait IntoRustByName<R> {
    fn get_by_name(&self, name: &str) -> Option<CDRSResult<R>>;
}

/// Tries to converts u64 numerical value into array of n bytes.
pub fn try_to_n_bytes(int: u64, n: usize) -> io::Result<Vec<u8>> {
    let mut bytes = vec![];
    try!(bytes.write_uint::<BigEndian>(int, n));

    return Ok(bytes);
}

/// Converts u64 numerical value into array of n bytes
pub fn to_n_bytes(int: u64, n: usize) -> Vec<u8> {
    return try_to_n_bytes(int, n).unwrap();
}

pub fn try_i_to_n_bytes(int: i64, n: usize) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::with_capacity(n);
    unsafe {
        bytes.set_len(n);
    }
    BigEndian::write_int(&mut bytes, int, n);

    return Ok(bytes);
}

pub fn i_to_n_bytes(int: i64, n: usize) -> Vec<u8> {
    return try_i_to_n_bytes(int, n).unwrap();
}

///
pub fn try_from_bytes(bytes: Vec<u8>) -> Result<u64, io::Error> {
    let mut c = Cursor::new(bytes.clone());
    return c.read_uint::<BigEndian>(bytes.len());
}

///
pub fn try_u16_from_bytes(bytes: Vec<u8>) -> Result<u16, io::Error> {
    let mut c = Cursor::new(bytes.clone());
    return c.read_u16::<BigEndian>();
}

///
pub fn try_i_from_bytes(bytes: Vec<u8>) -> Result<i64, io::Error> {
    let mut c = Cursor::new(bytes.clone());
    return c.read_int::<BigEndian>(bytes.len());
}

///
pub fn try_i32_from_bytes(bytes: Vec<u8>) -> Result<i32, io::Error> {
    let mut c = Cursor::new(bytes.clone());
    return c.read_i32::<BigEndian>();
}

///
pub fn try_f32_from_bytes(bytes: Vec<u8>) -> Result<f32, io::Error> {
    let mut c = Cursor::new(bytes.clone());
    return c.read_f32::<BigEndian>();
}

///
pub fn try_f64_from_bytes(bytes: Vec<u8>) -> Result<f64, io::Error> {
    let mut c = Cursor::new(bytes.clone());
    return c.read_f64::<BigEndian>();
}

/// Converts byte-array into u64
pub fn from_bytes(bytes: Vec<u8>) -> u64 {
    return try_from_bytes(bytes).unwrap();
}

/// Converts byte-array into i64
pub fn from_i_bytes(bytes: Vec<u8>) -> i64 {
    return try_i_from_bytes(bytes).unwrap();
}

/// Converts byte-array into u16
pub fn from_u16_bytes(bytes: Vec<u8>) -> u16 {
    return try_u16_from_bytes(bytes).unwrap();
}

/// Converts number u64 into Cassandra's [short].
pub fn to_short(int: u64) -> Vec<u8> {
    return to_n_bytes(int, SHORT_LEN);
}

/// Convers integer into Cassandra's [int]
pub fn to_int(int: i64) -> Vec<u8> {
    return i_to_n_bytes(int, INT_LEN);
}

#[derive(Debug, Clone)]
pub struct CString {
    string: String,
}

impl CString {
    pub fn new(string: String) -> CString {
        return CString { string: string };
    }

    /// Converts internal value into pointer of `str`.
    pub fn as_str<'a>(&'a self) -> &'a str {
        return self.string.as_str();
    }

    /// Converts internal value into a plain `String`.
    pub fn into_plain(self) -> String {
        return self.string;
    }

    /// Represents internal value as a `String`.
    pub fn as_plain(&self) -> String {
        return self.string.clone();
    }
}

// Implementation for Rust std types
// Use extended Rust string as Cassandra [string]
impl IntoBytes for CString {
    /// Converts into Cassandra byte representation of [string]
    fn into_cbytes(&self) -> Vec<u8> {
        let mut v: Vec<u8> = vec![];
        let l = self.string.len() as u64;
        v.extend_from_slice(to_short(l).as_slice());
        v.extend_from_slice(self.string.as_bytes());
        return v;
    }
}

impl FromCursor for CString {
    /// from_cursor gets Cursor who's position is set such that it should be a start of a [string].
    /// It reads required number of bytes and returns a String
    fn from_cursor(mut cursor: &mut Cursor<Vec<u8>>) -> CString {
        let len_bytes = cursor_next_value(&mut cursor, SHORT_LEN as u64);
        let len: u64 = from_bytes(len_bytes.to_vec());
        let body_bytes = cursor_next_value(&mut cursor, len);

        return CString { string: String::from_utf8(body_bytes).unwrap() };
    }
}

#[derive(Debug, Clone)]
pub struct CStringLong {
    string: String,
}

impl CStringLong {
    pub fn new(string: String) -> CStringLong {
        return CStringLong { string: string };
    }

    /// Converts internal value into pointer of `str`.
    pub fn as_str<'a>(&'a self) -> &'a str {
        return self.string.as_str();
    }

    /// Converts internal value into a plain `String`.
    pub fn into_plain(self) -> String {
        return self.string;
    }
}

// Implementation for Rust std types
// Use extended Rust string as Cassandra [string]
impl IntoBytes for CStringLong {
    /// Converts into Cassandra byte representation of [string]
    fn into_cbytes(&self) -> Vec<u8> {
        let mut v: Vec<u8> = vec![];
        let l = self.string.len() as i64;
        v.extend_from_slice(to_int(l).as_slice());
        v.extend_from_slice(self.string.as_bytes());
        return v;
    }
}

impl FromCursor for CStringLong {
    /// from_cursor gets Cursor who's position is set such that it should be a start of a [string].
    /// It reads required number of bytes and returns a String
    fn from_cursor(mut cursor: &mut Cursor<Vec<u8>>) -> CStringLong {
        let len_bytes = cursor_next_value(&mut cursor, INT_LEN as u64);
        let len: u64 = from_bytes(len_bytes.to_vec());
        let body_bytes = cursor_next_value(&mut cursor, len);

        return CStringLong { string: String::from_utf8(body_bytes).unwrap() };
    }
}

#[derive(Debug, Clone)]
pub struct CStringList {
    pub list: Vec<CString>,
}

impl CStringList {
    pub fn into_plain(self) -> Vec<String> {
        return self.list
            .iter()
            .map(|string| string.clone().into_plain())
            .collect();
    }
}

impl IntoBytes for CStringList {
    fn into_cbytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        let l = to_short(self.list.len() as u64);
        bytes.extend_from_slice(l.as_slice());

        bytes = self.list
            .iter()
            .fold(bytes, |mut _bytes, cstring| {
                _bytes.extend_from_slice(cstring.into_cbytes().as_slice());
                _bytes
            });

        return bytes;
    }
}

impl FromCursor for CStringList {
    fn from_cursor(mut cursor: &mut Cursor<Vec<u8>>) -> CStringList {
        let mut len_bytes = [0; SHORT_LEN];
        if let Err(err) = cursor.read(&mut len_bytes) {
            error!("Read Cassandra bytes error: {}", err);
            panic!(err);
        }
        let len: u64 = from_bytes(len_bytes.to_vec());
        let list = (0..len).map(|_| CString::from_cursor(&mut cursor)).collect();
        return CStringList { list: list };
    }
}

//

#[derive(Debug, Clone)]
/// The structure that represents Cassandra byte type
pub struct CBytes {
    bytes: Vec<u8>,
}

impl CBytes {
    pub fn new(bytes: Vec<u8>) -> CBytes {
        return CBytes { bytes: bytes };
    }
    /// Converts `CBytes` into a plain array of bytes
    pub fn into_plain(self) -> Vec<u8> {
        return self.bytes;
    }
    pub fn as_plain(&self) -> Vec<u8> {
        return self.bytes.clone();
    }
}

impl FromCursor for CBytes {
    /// from_cursor gets Cursor who's position is set such that it should be a start of a [bytes].
    /// It reads required number of bytes and returns a CBytes
    fn from_cursor(mut cursor: &mut Cursor<Vec<u8>>) -> CBytes {
        let len: u64 = CInt::from_cursor(&mut cursor) as u64;
        return CBytes { bytes: cursor_next_value(&mut cursor, len) };
    }
}

// Use extended Rust Vec<u8> as Cassandra [bytes]
impl IntoBytes for CBytes {
    fn into_cbytes(&self) -> Vec<u8> {
        let mut v: Vec<u8> = vec![];
        let l = self.bytes.len() as i64;
        v.extend_from_slice(to_int(l).as_slice());
        v.extend_from_slice(self.bytes.as_slice());
        return v;
    }
}

/// Cassandra short bytes
#[derive(Debug, Clone)]
pub struct CBytesShort {
    bytes: Vec<u8>,
}

impl CBytesShort {
    pub fn new(bytes: Vec<u8>) -> CBytesShort {
        return CBytesShort { bytes: bytes };
    }
    /// Converts `CBytesShort` into plain vector of bytes;
    pub fn into_plain(self) -> Vec<u8> {
        return self.bytes;
    }
}

impl FromCursor for CBytesShort {
    /// from_cursor gets Cursor who's position is set such that it should be a start of a [bytes].
    /// It reads required number of bytes and returns a CBytes
    fn from_cursor(mut cursor: &mut Cursor<Vec<u8>>) -> CBytesShort {
        let len: u64 = CIntShort::from_cursor(&mut cursor) as u64;
        return CBytesShort { bytes: cursor_next_value(&mut cursor, len) };
    }
}

// Use extended Rust Vec<u8> as Cassandra [bytes]
impl IntoBytes for CBytesShort {
    fn into_cbytes(&self) -> Vec<u8> {
        let mut v: Vec<u8> = vec![];
        let l = self.bytes.len() as u64;
        v.extend_from_slice(to_short(l).as_slice());
        v.extend_from_slice(self.bytes.as_slice());
        return v;
    }
}


/// Cassandra int type.
pub type CInt = i32;

impl FromCursor for CInt {
    fn from_cursor(mut cursor: &mut Cursor<Vec<u8>>) -> CInt {
        let bytes = cursor_next_value(&mut cursor, INT_LEN as u64);
        return from_bytes(bytes) as CInt;
    }
}

/// Cassandra int short type.
pub type CIntShort = i16;

impl FromCursor for CIntShort {
    fn from_cursor(mut cursor: &mut Cursor<Vec<u8>>) -> CIntShort {
        let bytes = cursor_next_value(&mut cursor, SHORT_LEN as u64);
        return from_bytes(bytes) as CIntShort;
    }
}

// Use extended Rust Vec<u8> as Cassandra [bytes]
impl FromBytes for Vec<u8> {
    fn from_bytes(bytes: Vec<u8>) -> Vec<u8> {
        let mut cursor = Cursor::new(bytes);
        let len_bytes = cursor_next_value(&mut cursor, SHORT_LEN as u64);
        let len: u64 = from_bytes(len_bytes);
        return cursor_next_value(&mut cursor, len);
    }
}

/// The structure wich represets Cassandra [inet]
/// (https://github.com/apache/cassandra/blob/trunk/doc/native_protocol_v4.spec#L222).
#[derive(Debug)]
pub struct CInet {
    pub addr: SocketAddr,
}

impl FromCursor for CInet {
    fn from_cursor(mut cursor: &mut Cursor<Vec<u8>>) -> CInet {
        let n = CIntShort::from_cursor(&mut cursor);
        let bytes = cursor_next_value(&mut cursor, n as u64);
        let ip = decode_inet(bytes).unwrap();
        let port = CInt::from_cursor(&mut cursor);
        let socket_addr = SocketAddr::new(ip, port as u16);

        CInet { addr: socket_addr }
    }
}

pub fn cursor_next_value(mut cursor: &mut Cursor<Vec<u8>>, len: u64) -> Vec<u8> {
    let l = len as usize;
    let current_position = cursor.position();
    let mut buff: Vec<u8> = Vec::with_capacity(l);
    unsafe {
        buff.set_len(l);
    }
    if let Err(err) = cursor.read(&mut buff) {
        error!("Read from cursor error: {}", err);
        panic!(err);
    }
    cursor.set_position(current_position + len);
    return buff;
}


#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::*;
    use {IntoBytes, FromCursor};

    // CString
    #[test]
    fn test_cstring_new() {
        let foo = "foo".to_string();
        let _ = CString::new(foo);
    }

    #[test]
    fn test_cstring_as_str() {
        let foo = "foo".to_string();
        let cstring = CString::new(foo);

        assert_eq!(cstring.as_str(), "foo");
    }

    #[test]
    fn test_cstring_into_plain() {
        let foo = "foo".to_string();
        let cstring = CString::new(foo);

        assert_eq!(cstring.into_plain(), "foo".to_string());
    }

    #[test]
    fn test_cstring_into_cbytes() {
        let foo = "foo".to_string();
        let cstring = CString::new(foo);

        assert_eq!(cstring.into_cbytes(), vec![0, 3, 102, 111, 111]);
    }

    #[test]
    fn test_cstring_from_cursor() {
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(vec![0, 3, 102, 111, 111, 0]);
        let cstring = CString::from_cursor(&mut cursor);
        println!("{:?}", &cursor);
        assert_eq!(cstring.as_str(), "foo");
    }

    // CStringLong
    #[test]
    fn test_cstringlong_new() {
        let foo = "foo".to_string();
        let _ = CStringLong::new(foo);
    }

    #[test]
    fn test_cstringlong_as_str() {
        let foo = "foo".to_string();
        let cstring = CStringLong::new(foo);

        assert_eq!(cstring.as_str(), "foo");
    }

    #[test]
    fn test_cstringlong_into_plain() {
        let foo = "foo".to_string();
        let cstring = CStringLong::new(foo);

        assert_eq!(cstring.into_plain(), "foo".to_string());
    }

    #[test]
    fn test_cstringlong_into_cbytes() {
        let foo = "foo".to_string();
        let cstring = CStringLong::new(foo);

        assert_eq!(cstring.into_cbytes(), vec![0, 0, 0, 3, 102, 111, 111]);
    }

    #[test]
    fn test_cstringlong_from_cursor() {
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(vec![0, 0, 0, 3, 102, 111, 111, 0]);
        let cstring = CStringLong::from_cursor(&mut cursor);
        println!("{:?}", &cursor);
        assert_eq!(cstring.as_str(), "foo");
    }

    // CStringList
    #[test]
    fn test_cstringlist() {
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(vec![0, 2, 0, 3, 102, 111, 111, 0, 3, 102,
                                                           111, 111]);
        let list = CStringList::from_cursor(&mut cursor);
        let plain = list.into_plain();
        assert_eq!(plain.len(), 2);
        for s in plain.iter() {
            assert_eq!(s.as_str(), "foo");
        }
    }

    // CBytes
    #[test]
    fn test_cbytes_new() {
        let bytes_vec: Vec<u8> = vec![1, 2, 3];
        let _ = CBytes::new(bytes_vec);
    }

    #[test]
    fn test_cbytes_into_plain() {
        let cbytes = CBytes::new(vec![1, 2, 3]);
        assert_eq!(cbytes.into_plain(), vec![1, 2, 3]);
    }

    #[test]
    fn test_cbytes_from_cursor() {
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(vec![0, 0, 0, 3, 1, 2, 3]);
        let cbytes = CBytes::from_cursor(&mut cursor);
        assert_eq!(cbytes.into_plain(), vec![1, 2, 3]);
    }

    #[test]
    fn test_cbytes_into_cbytes() {
        let bytes_vec: Vec<u8> = vec![1, 2, 3];
        let cbytes = CBytes::new(bytes_vec);
        assert_eq!(cbytes.into_cbytes(), vec![0, 0, 0, 3, 1, 2, 3]);
    }

    // CBytesShort
    #[test]
    fn test_cbytesshort_new() {
        let bytes_vec: Vec<u8> = vec![1, 2, 3];
        let _ = CBytesShort::new(bytes_vec);
    }

    #[test]
    fn test_cbytesshort_into_plain() {
        let cbytes = CBytesShort::new(vec![1, 2, 3]);
        assert_eq!(cbytes.into_plain(), vec![1, 2, 3]);
    }

    #[test]
    fn test_cbytesshort_from_cursor() {
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(vec![0, 3, 1, 2, 3]);
        let cbytes = CBytesShort::from_cursor(&mut cursor);
        assert_eq!(cbytes.into_plain(), vec![1, 2, 3]);
    }

    #[test]
    fn test_cbytesshort_into_cbytes() {
        let bytes_vec: Vec<u8> = vec![1, 2, 3];
        let cbytes = CBytesShort::new(bytes_vec);
        assert_eq!(cbytes.into_cbytes(), vec![0, 3, 1, 2, 3]);
    }

    // CInt
    #[test]
    fn test_cint_from_cursor() {
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(vec![0, 0, 0, 5]);
        let i = CInt::from_cursor(&mut cursor);
        assert_eq!(i, 5);
    }

    // CIntShort
    #[test]
    fn test_cintshort_from_cursor() {
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(vec![0, 5]);
        let i = CIntShort::from_cursor(&mut cursor);
        assert_eq!(i, 5);
    }

    // cursor_next_value
    #[test]
    fn test_cursor_next_value() {
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(vec![0, 1, 2, 3, 4]);
        let l: u64 = 3;
        let val = cursor_next_value(&mut cursor, l);
        assert_eq!(val, vec![0, 1, 2]);
    }

}
