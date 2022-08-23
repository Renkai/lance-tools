use crate::protos::Metadata as MetaPB;
use prost::Message;
use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug)]
pub struct Metadata {
    pb: MetaPB,
}

use std::io::{Cursor, Read, Seek, SeekFrom};

pub fn get_schema<R: Read + Seek>(file: &mut R, pos: i64) -> Metadata {
    // let mut str = String::new();
    // file.read_to_string(&mut str).unwrap();
    // println!("str: {:?}", str);

    let mut size_buf = [0; 4];
    file.seek(SeekFrom::Start(pos as u64)).unwrap();
    file.read_exact(&mut size_buf).unwrap();
    let pb_size = Cursor::new(size_buf).read_i32::<LittleEndian>().unwrap();
    let mut buf = vec![0; pb_size as usize];
    file.read_exact(&mut buf).unwrap();
    let pb = MetaPB::decode(&*buf).unwrap();
    return Metadata {
        pb
    };
}
