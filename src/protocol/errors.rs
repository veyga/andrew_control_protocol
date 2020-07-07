use backtrace::Backtrace;
use std::io::{Error, ErrorKind};
use std::sync::mpsc::{RecvError, SendError};

#[derive(Debug)]
pub struct TracedError {
    pub msg: String,
    pub backtrace: Backtrace,
}

impl TracedError {
    pub fn new(msg: String) -> TracedError {
        TracedError {
            msg: msg.to_string(),
            backtrace: Backtrace::new(),
        }
    }
}

#[derive(Debug)]
pub enum ACPError {
    Io(TracedError),
    Channel(ChannelError),
    Ref(TracedError),
}

#[derive(Debug)]
pub enum ChannelError {
    Send(TracedError),
    Recv(TracedError),
}

impl From<ChannelError> for ACPError {
    fn from(e: ChannelError) -> ACPError {
        ACPError::Channel(e)
    }
}

impl From<std::io::Error> for ACPError {
    fn from(e: std::io::Error) -> ACPError {
        ACPError::Io(TracedError::new(e.to_string()))
    }
}

impl From<RecvError> for ACPError {
    fn from(e: RecvError) -> ACPError {
        ACPError::Channel(ChannelError::Recv(TracedError::new(e.to_string())))
    }
}

impl From<SendError<Vec<u8>>> for ACPError {
    fn from(e: SendError<Vec<u8>>) -> ACPError {
        ACPError::Channel(ChannelError::Send(TracedError::new(e.to_string())))
    }
}

