use super::{CHUNK_SIZE, ACPError};
use std::fs::File;
use std::io::{BufReader, Read};
use std::sync::mpsc::Sender;

pub fn read_loop(infile: &str, channel_send: Sender<Vec<u8>>) -> Result<(), ACPError> {
    let file = File::open(infile)?;
    let mut reader = BufReader::with_capacity(CHUNK_SIZE, file);
    let mut buffer = [0u8; CHUNK_SIZE];
    let mut total_bytes_read = 0;
    loop {
        let num_bytes_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                total_bytes_read += n;
                n
            }
            Err(_) => break,
        };

        // send buffer to send thread/ exit if error
        channel_send.send(Vec::from(&buffer[..num_bytes_read]))?;
    }

    //send empty buffer to notify sender that reading is done
    let _ = channel_send.send(Vec::new());
    println!("Total bytes read {}", total_bytes_read);
    Ok(())
}
