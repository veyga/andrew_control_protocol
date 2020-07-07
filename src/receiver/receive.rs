use super::CHUNK_SIZE;
use crate::protocol::*;
use std::collections::HashSet;
use std::net::{Ipv4Addr, UdpSocket};
use std::rc::Rc;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

pub fn receive_loop(
    listening_port: String,
    window_size: String,
    channel_send: Sender<Vec<u8>>,
) -> Result<(), ACPError> {
    let socket_addr = Ipv4Addr::LOCALHOST.to_string() + ":" + &listening_port;
    let socket = UdpSocket::bind(&socket_addr)?;
    println!("ACP RECEIVER Running. bound to {}", socket_addr);
    let mut win = window_size.parse::<u8>().unwrap_or_else(|i| 1);
    let mut seq: u8 = 0;
    let mut max = seq + win;

    let build_acknowledgement = |seq, win| {
        SegmentBuilder::new()
            .seg_type(SegmentType::Ack)
            .sequence(seq)
            .win(win)
            .build()
    };

    'receive: loop {
        let mut buffer = [0_u8; MAX_SEGMENT_SIZE];
        let (datagram_size, origin) = socket.recv_from(&mut buffer)?;
        let (header, rc_payload) = Segment::from(Vec::from(&buffer[..datagram_size])).split();
        let dg_seq = header.sequence;
        if (seq..(seq + win)).contains(&dg_seq) {
            if let Ok(payload) = Rc::try_unwrap(rc_payload) {
                if dg_seq != seq {
                    let ack = build_acknowledgement(seq, win); 
                    println!("sending dup ack (out of order seg received){}", ack.header);
                    socket.send_to(&Vec::from(ack), origin)?;
                } else {
                    println!("received {}", header);
                    let payload_len = payload.len();
                    channel_send.send(payload)?;
                    seq += 1;
                    if seq == max {
                        win = rand_win()();
                        max += win;
                    }
                    let ack = build_acknowledgement(seq, win);
                    println!("sending {}", ack.header);
                    socket.send_to(&Vec::from(ack), origin)?;
                    if payload_len < MAX_PAYLOAD_SIZE {
                        break 'receive;
                    }
                }
            }
        }
    }
    Ok(())
}
