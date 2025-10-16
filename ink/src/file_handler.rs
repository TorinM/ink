use std::env;
use std::fs::File;
use std::io::{Seek, SeekFrom, Write};

pub fn overwrite_file(file: &mut File, data_buf: &Vec<u8>) -> Result<(), std::io::Error> {
    file.seek(SeekFrom::Start(0)).unwrap();
    file.write_all(data_buf)
}

pub fn get_file_name() -> Result<String, std::io::Error> {
    let args: Vec<String> = env::args().collect();

    let file_name = &args[1];

    Ok(file_name.to_string())
}
