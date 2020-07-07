use acp::protocol::ACPError;
use acp::receiver::{args::Args, receive, write};
use std::sync::mpsc;
use std::thread;

#[allow(unused_imports, unused_variables)]
fn main() -> Result<(), ACPError> {
    let Args {
        listening_port,
        window_size,
        outfile,
    } = Args::parse();

    let (channel_send, channel_receive) = mpsc::channel();
    let receive_thread =
        thread::spawn(move || receive::receive_loop(listening_port, window_size, channel_send));
    let write_thread = thread::spawn(move || write::write_loop(&outfile, channel_receive));
    let receive_io_result = receive_thread.join().unwrap();
    let write_io_result = write_thread.join().unwrap();
    receive_io_result?;
    write_io_result?;
    Ok(())
}
