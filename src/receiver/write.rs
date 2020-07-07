use super::{CHUNK_SIZE, ACPError};
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::mpsc::Receiver;

pub fn write_loop(outfile: &str, channel_receive: Receiver<Vec<u8>>) -> Result<(), ACPError> {
    let outpath = Path::new(outfile);
    if outpath.exists() {
        fs::remove_file(outpath)?;
    }

    let file = File::create(outfile)?;
    let mut writer = Box::new(BufWriter::with_capacity(CHUNK_SIZE, file));
    let mut total_bytes_written = 0;
    loop {
        let payload = channel_receive.recv()?;
        total_bytes_written += payload.len();
        writer.write_all(&payload)?;
        if payload.len() < CHUNK_SIZE {
            println!("Total bytes written {}", total_bytes_written);
            break;
        }
    }
    Ok(())
}
