pub struct GpxFileParser {
    iter: Bytes<BufReader<File>>
}

impl GpxFileParser {

    ///
    /// Creates a new GpxParser struct.
    ///
    pub fn new() -> GpxParser {

    }

    ///
    ///
    ///
    fn parse(self, gpx_file: BufReader<File>) /* -> Vec<GpxTrackSegment> */ {
        let bytes_buf_reader_iter : Bytes<BufReader<File>> = gpx_file.bytes();

        while let Some(Ok(byte)) = bytes_buf_reader_iter.next() {
            if is_gt_byte(byte) {
                parse_trksgt(&bytes_buf_reader_iter);
            }
        }
    }

    fn parse_trksgt(self, bytes_buf_reader_iter: &Bytes<BufReader<File>>) -> Option<GpxTrackSegment> {
        let trkseg : &str = "trkseg";

        for byte in trkseg.bytes() {
            if let Some(Ok(next_byte)) = bytes_buf_reader_iter.next() {
                if byte != next_byte {
                    return None;
                }
            }
        }

        if let Some(Ok(next_byte)) = bytes_buf_reader_iter.next() {
            if !is_lt_byte(next_byte) {
                return None;
            }
        }

        Some(GpxTrackSegment::new())
    }

    fn is_gt_byte(self, byte: u8) -> bool {
        byte == 0x003C
    }

    fn is_lt_byte(self, byte: u8) -> bool {
        byte == 0x003E
    }

    fn parse_gt(self) {

    }

    fn parse_lt(self) {

    }
}
