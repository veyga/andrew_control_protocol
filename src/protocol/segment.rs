use crate::protocol::constants::*;
use rand::Rng;
use std::fmt;
use std::rc::Rc;

pub fn rand_win() -> impl Fn() -> u8 {
    move || rand::thread_rng().gen_range(1u8, MAX_WINDOW_SIZE as u8)
}

#[derive(Debug, Clone, Hash)]
pub struct Segment {
    pub header: Header,
    pub payload: Rc<Vec<u8>>,
}

#[derive(Default, Debug, Clone, Hash)]
pub struct Header {
    pub type_win: u8,
    pub sequence: u8,
    pub length_b1: u8,
    pub length_b2: u8,
}

#[derive(Debug, PartialEq)]
pub enum SegmentType {
    Data,
    Ack,
}

pub struct SegmentBuilder {
    type_win: u8,
    sequence: u8,
    length_b1: u8,
    length_b2: u8,
    payload: Rc<Vec<u8>>,
}

impl Segment {
    pub fn split(&self) -> (Header, Rc<Vec<u8>>) {
        (self.header.clone(), self.payload.clone())
    }
}

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        self.header.sequence == other.header.sequence
    }
}
impl Eq for Segment {}

impl PartialOrd for Segment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Segment {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.header.sequence.cmp(&other.header.sequence)
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ACPHeader: type: {:?}, win: {}, seq: {}, length: {}",
            self.segment_type(),
            self.window(),
            self.sequence,
            self.length()
        )
    }
}

impl Header {
    pub fn segment_type(&self) -> SegmentType {
        match self.type_win & 0b11111000 {
            0b00001000 => SegmentType::Data,
            _ => SegmentType::Ack,
        }
    }

    pub fn window(&self) -> u8 {
        self.type_win & 0b00000111
    }

    pub fn length(&self) -> u16 {
        let shft = (self.length_b1 as u16) << 8;
        let grow = self.length_b2 as u16;
        shft ^ grow
    }
}

impl SegmentBuilder {
    pub fn new() -> SegmentBuilder {
        SegmentBuilder {
            type_win: 0,
            sequence: 0,
            length_b1: 0,
            length_b2: 0,
            payload: Rc::new(Vec::new()),
        }
    }

    pub fn seg_type(&mut self, t: SegmentType) -> &mut SegmentBuilder {
        match t {
            SegmentType::Data => {
                self.type_win ^= 0b00001000;
                self
            }
            SegmentType::Ack => {
                self.type_win ^= 0b00010000;
                self
            }
        }
    }

    pub fn win(&mut self, window: u8) -> &mut SegmentBuilder {
        assert!(window <= MAX_WINDOW_SIZE as u8);
        self.type_win ^= window;
        self
    }

    pub fn type_win(&mut self, type_win: u8) -> &mut SegmentBuilder {
        self.type_win = type_win;
        self
    }

    pub fn sequence(&mut self, sequence: u8) -> &mut SegmentBuilder {
        self.sequence = sequence;
        self
    }

    pub fn length_b1(&mut self, length_b1: u8) -> &mut SegmentBuilder {
        self.length_b1 = length_b1;
        self
    }

    pub fn length_b2(&mut self, length_b2: u8) -> &mut SegmentBuilder {
        self.length_b2 = length_b2;
        self
    }

    pub fn length(&mut self, length: u16) -> &mut SegmentBuilder {
        self.length_b2 = (0b0000000011111111 & length) as u8;
        self.length_b1 = ((0b1111111100000000 & length) >> 8) as u8;
        self
    }

    pub fn payload(&mut self, payload: Rc<Vec<u8>>) -> &mut SegmentBuilder {
        self.payload = payload;
        self
    }

    pub fn build(&self) -> Segment {
        let header = Header {
            type_win: self.type_win,
            sequence: self.sequence,
            length_b1: self.length_b1,
            length_b2: self.length_b2,
        };
        Segment {
            header,
            payload: self.payload.clone(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn acknowledgement_segment() {
        let segment = SegmentBuilder::new()
            .seg_type(SegmentType::Ack)
            .win(1)
            .sequence(5)
            .build();
        assert_eq!(segment.header.segment_type(), SegmentType::Ack);
        assert_eq!(segment.header.window(), 1);
        assert_eq!(segment.header.sequence, 5);
        assert_eq!(segment.header.length(), 0);
    }

    #[test]
    fn data_full_segment() {
        let data = [0u8; MAX_PAYLOAD_SIZE];
        let segment = SegmentBuilder::new()
            .seg_type(SegmentType::Data)
            .win(2)
            .sequence(3)
            .length(MAX_PAYLOAD_SIZE as u16)
            .payload(Rc::new(Vec::from(&data[..MAX_PAYLOAD_SIZE])))
            .build();
        assert_eq!(segment.header.segment_type(), SegmentType::Data);
        assert_eq!(segment.header.window(), 2);
        assert_eq!(segment.header.sequence, 3);
        assert_eq!(segment.header.length(), 512);
    }

    #[test]
    fn data_non_full_segment() {
        const HALF: usize = MAX_PAYLOAD_SIZE / 2;
        let data = [0u8; HALF];
        let segment = SegmentBuilder::new()
            .seg_type(SegmentType::Data)
            .win(2)
            .sequence(3)
            .length(HALF as u16)
            .payload(Rc::new(Vec::from(&data[..HALF])))
            .build();
        assert_eq!(segment.header.segment_type(), SegmentType::Data);
        assert_eq!(segment.header.length(), HALF as u16);
    }

    #[test]
    fn test_header_display() {
        let segment = SegmentBuilder::new()
            .seg_type(SegmentType::Data)
            .win(1)
            .sequence(5)
            .length(20)
            .build();
        let expected = "ACPHeader: type: Data, win: 1, seq: 5, length: 20";
        assert_eq!(expected, format!("{}", segment.header));
    }

    #[test]
    fn test_header_debug() {
        let segment = SegmentBuilder::new()
            .seg_type(SegmentType::Data)
            .win(1)
            .sequence(5)
            .length(20)
            .build();
        let expected = "Header { type_win: 9, sequence: 5, length_b1: 0, length_b2: 20 }";
        assert_eq!(expected, format!("{:?}", segment.header));
    }

    #[test]
    fn test_header_display_ack() {
        let segment = SegmentBuilder::new()
            .seg_type(SegmentType::Ack)
            .win(1)
            .sequence(3)
            .length(0)
            .build();
        let expected = "ACPHeader: type: Ack, win: 1, seq: 3, length: 0";
        assert_eq!(expected, format!("{}", segment.header));
    }

    #[test]
    fn test_segments_can_be_sorted() {
        let segment1 = SegmentBuilder::new()
            .seg_type(SegmentType::Ack)
            .win(3)
            .sequence(1)
            .length(0)
            .build();

        let segment2 = SegmentBuilder::new()
            .seg_type(SegmentType::Ack)
            .win(3)
            .sequence(2)
            .length(0)
            .build();
        let segment3 = SegmentBuilder::new()
            .seg_type(SegmentType::Ack)
            .win(3)
            .sequence(3)
            .length(0)
            .build();

        use std::collections::BTreeSet;
        let mut bset: BTreeSet<_> = BTreeSet::new();
        bset.insert(segment1);
        bset.insert(segment3);
        bset.insert(segment2);
        let mut i = 1;
        for s in bset {
            assert_eq!(i, s.header.sequence);
            i += 1;
        }
    }
}
