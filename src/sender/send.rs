use super::CHUNK_SIZE;
use crate::protocol::*;
use std::collections::HashSet;
use std::io::ErrorKind;
use std::net::{Ipv4Addr, UdpSocket};
use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

pub fn send_loop(
    destination_name: String,
    destination_port: String,
    window_size: String,
    channel_receive: Receiver<Vec<u8>>,
) -> Result<(), ACPError> {
    println!("ACP Sender Running");
    let source_port = destination_port.parse::<u16>().map(|p| p - 1).unwrap();
    let socket_addr = Ipv4Addr::LOCALHOST.to_string() + ":" + &source_port.to_string();
    let socket = UdpSocket::bind(&socket_addr).expect("Failed to bind ACP Sender socket");
    let receiver_addr = destination_name + ":" + &destination_port;
    let mut win = window_size.parse::<u8>().unwrap_or_else(|i| 1);
    let mut seq: u8 = 0;
    let mut last_sequence: Option<u8> = None;

    'channel: loop {
        let mut segments = Vec::new();
        let mut curr = seq;
        for i in 0..win {
            let buffer = channel_receive.recv().unwrap();
            if buffer.is_empty() {
                last_sequence = Some(curr - 1);
                break;
            }
            let segment = SegmentBuilder::new()
                .seg_type(SegmentType::Data)
                .win(win)
                .sequence(curr)
                .length(buffer.len() as u16)
                .payload(Rc::new(buffer))
                .build();
            segments.push(segment);
            curr += 1;
        }

        let mut n: u8 = 0;
        let stop = segments.len() as u8;
        'segment: loop {
            let max = seq + win;
            for i in n..stop {
                let seg = segments[i as usize].clone();
                println!("sending {}", seg.header);
                let bytes_sent = socket.send_to(&Vec::from(seg), &receiver_addr)?;
            }

            'confirm: loop {
                socket.set_read_timeout(Some(Duration::from_secs(6)))?;
                let mut buffer = [0u8; HEADER_SIZE];
                match socket.recv(&mut buffer) {
                    Err(e) => {
                        n = max - seq;
                        println!("No ACK.. resending");
                        break 'confirm; //go back N
                    }
                    Ok(size) => {
                        let (header, _) = Segment::from(Vec::from(&buffer[..size])).split();
                        println!("received {}", header);
                        match last_sequence {
                            None => {
                                if (header.sequence - 1) == seq {
                                    seq += 1;
                                    if seq == max {
                                        win = header.window();
                                        thread::sleep(Duration::from_secs(2));
                                        break 'segment;
                                    }
                                }
                            }
                            Some(last) => {
                                if last == (header.sequence - (1_u8)) {
                                    break 'channel;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
