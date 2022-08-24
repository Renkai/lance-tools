use crate::protos::Metadata as MetaPB;
use prost::Message;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

#[derive(Debug)]
pub struct Metadata {
    pb: MetaPB,
}

use std::io::{BufReader, Cursor, Read, Seek, SeekFrom};

pub fn get_schema<R: Read + Seek>(file: &mut R) -> Metadata {
    // let mut str = String::new();
    // println!("str: {:?}", str);

    let mut size_buf = [0; 4];
    file.read_exact(&mut size_buf).unwrap();
    let pb_size = Cursor::new(size_buf).read_i32::<LittleEndian>().unwrap();
    println!("pb_size: {:?}", pb_size);
    // let mut buf = vec![0; pb_size as usize];
    // file.seek(SeekFrom::Start(4 as u64)).unwrap();
    // let mut bufReader = BufReader::new(file);
    bufReader.seek(SeekFrom::Start(4)).unwrap();
    let buffer = bufReader.buffer();
    // file.read_exact(&mut buf).unwrap();
    let pb = MetaPB::decode_length_delimited(buffer).unwrap();
    return Metadata {
        pb
    };
}
