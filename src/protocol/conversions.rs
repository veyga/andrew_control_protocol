use crate::protocol::segment::*;
use crate::protocol::HEADER_SIZE;
use std::rc::Rc;

impl From<Segment> for Vec<u8> {
    fn from(segment: Segment) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::with_capacity(segment.payload.len() + HEADER_SIZE);
        vec.push(segment.header.type_win);
        vec.push(segment.header.sequence);
        vec.push(segment.header.length_b1);
        vec.push(segment.header.length_b2);
        if let Some(ptr) = segment.payload.get(..) {
            vec.extend_from_slice(ptr);
        }
        vec
    }
}

impl From<Vec<u8>> for Segment {
    fn from(buffer: Vec<u8>) -> Self {
        let (header, payload) = buffer.split_at(HEADER_SIZE);
        SegmentBuilder::new()
            .type_win(buffer[0])
            .sequence(buffer[1])
            .length_b1(buffer[2])
            .length_b2(buffer[3])
            .payload(Rc::new(Vec::from(payload)))
            .build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn segment_from_bytes_data() {
        let bytes: Vec<u8> = vec![9, 0, 0, 2, 100, 99];
        let segment = Segment::from(bytes);
        assert_eq!(segment.header.segment_type(), SegmentType::Data);
        assert_eq!(segment.header.window(), 1);
        assert_eq!(segment.header.sequence, 0);
        assert_eq!(segment.header.length(), 2);
    }

    #[test]
    fn bytes_from_segment_data() {
        let payload = vec![100,99];
        let segment = SegmentBuilder::new()
            .seg_type(SegmentType::Data)
            .win(1)
            .sequence(0)
            .length(2)
            .payload(Rc::new(payload))
            .build();
        let bytes = Vec::from(segment);
        assert_eq!(vec![9, 0, 0, 2, 100, 99], bytes);
    }

    #[test]
    fn segment_from_bytes_ack() {
        let bytes: Vec<u8> = vec![17, 5, 0, 0];
        let segment = Segment::from(bytes);
        assert_eq!(segment.header.segment_type(), SegmentType::Ack);
        assert_eq!(segment.header.window(), 1);
        assert_eq!(segment.header.sequence, 5);
        assert_eq!(segment.header.length(), 0);
    }

    #[test]
    fn bytes_from_segment_ack() {
        let segment = SegmentBuilder::new()
            .seg_type(SegmentType::Ack)
            .win(1)
            .sequence(5)
            .build();
        let bytes = Vec::from(segment);
        assert_eq!(vec![17, 5, 0, 0], bytes);
    }
}
