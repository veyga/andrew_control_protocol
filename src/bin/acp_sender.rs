#![allow(unused_imports, unused_variables)]
use acp::protocol::ACPError;
use acp::sender::{args::Args, read, send};
use std::sync::mpsc;
use std::thread;

fn main() -> Result<(), ACPError> {
    let Args {
        destination_name,
        destination_port,
        window_size,
        infile,
    } = Args::parse();

    let (channel_send, channel_receive) = mpsc::channel();

    let read_thread = thread::spawn(move || read::read_loop(&infile, channel_send));
    let send_thread = thread::spawn(move || {
        send::send_loop(
            destination_name,
            destination_port,
            window_size,
            channel_receive,
        )
    });

    let read_io_result = read_thread.join().unwrap();
    let send_io_result = send_thread.join().unwrap();
    read_io_result?;
    send_io_result?;
    Ok(())
}
