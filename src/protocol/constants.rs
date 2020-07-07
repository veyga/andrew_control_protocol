pub const MAX_PAYLOAD_SIZE: usize = 512;

pub const HEADER_SIZE: usize = 4;

pub const MAX_SEGMENT_SIZE: usize = MAX_PAYLOAD_SIZE + HEADER_SIZE;

pub const MAX_WINDOW_SIZE: usize = 7;

pub const TIMEOUT: usize = 3000;
