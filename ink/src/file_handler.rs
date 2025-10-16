use std::fs::File;
use std::io::{Seek, SeekFrom, Write};

pub fn overwrite_file(file: &mut File, data_buf: &Vec<u8>) -> Result<(), std::io::Error> {
    file.seek(SeekFrom::Start(0)).unwrap();
    file.write_all(data_buf)
}
