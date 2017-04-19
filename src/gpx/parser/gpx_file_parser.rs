use std::io::{BufReader, Bytes, Error, Read};
use std::fs::{File, ReadDir};
use std::iter::Peekable;
use super::super::GpxTrackSegment;
use super::GpxParserError;

///
///
///
pub struct GpxFileParser<T: Read> {
    iter: Peekable<Bytes<T>>,
    output: Vec<GpxTrackSegment>
}

impl<T: Read> GpxFileParser<T> {

    ///
    /// Creates a new GpxParser struct.
    ///
    /// Creates a peekable bytes iterator for the provided Read trait, then, while
    /// a stream of bytes exists, attempt to parse a <trkseg> tag.
    ///
    /// Returns `Result<Vec<GpxTrackSegment>, GpxParserError>` -
    ///
    pub fn parse(gpx_file_as_buf_reader: T) -> Result<Vec<GpxTrackSegment>, GpxParserError> {
        let parser = GpxFileParser {
            iter: gpx_file_as_buf_reader.bytes().peekable(),
            output: Vec::new()
        };

        while let Some(Ok(byte)) = parser.iter.next() {
            match parser.parse_trkseg_tag() {
                Some(Err(err)) => return Err(err),
            }
        }
        Ok(parser.output)
    }

    ///
    /// Parses a <trgseg> tag in a .gpx file.
    ///
    /// TRKSEG_TAG := <trkseg>
    ///
    fn parse_trkseg_tag(self) -> Option<Result<GpxTrackSegment, GpxParserError>> {
        // Parse <trkseg open tag, excluding the last less than, as a valid XML document
        // may contain as much whitepace as needed between the tag name and the less than
        // character.
        let trkseg_open_tag : &str = "<trkseg";

        for byte in trkseg_open_tag.bytes() {
            if let Some(Ok(next_byte)) = self.iter.next() {
                if byte != next_byte {
                    return None;
                }
            }
        }

        // We can safely assume that if we've reached this point, a trkseg element was intended,
        // so we create a GpxTrackSegment struct. Any errors from now will result in the return
        // of a GpxParserError.
        let gpx_track_segment : GpxTrackSegment = GpxTrackSegment::new();

        // Parse any whitespace between `<trgseg` and `>`.
        self.parse_whitespace();

        // Parse less than token
        if let Some(err) = self.parse_lt_token() {
            return Some(Err(err));
        }

        // Parse everything inside the trkseg
        self.parse_trkseg_tag_inner();

        let trkseg_close_tag : &str = "</trkseg>";
        for byte in trkseg_close_tag.bytes() {
            if let Some(Ok(next_byte)) = self.iter.next() {
                if byte != next_byte {
                    return None;
                }
            }
        }

        // If we've got to this point, we've successfully parsed a track segment and its containing
        // points, return it.
        None
    }

    ///
    /// Parses any whitespace where it exists in a .gpx file.
    ///
    /// Whitespace chars are ordered by the expected frequency they might be found in a standard
    /// .gpx document to enable if short-circuiting: Space, Carriage Return, Character Tabulation,
    /// Non-breaking Space, Line Feed, Form Feed, Next Line, Line Tabulation.
    ///
    /// This function has no need to return anything. It will consume up until it encounters EOF,
    /// or until it has consumed all whitespace characters.
    ///
    fn parse_whitespace(self) {
        while let Some(&Ok(next_byte)) = self.iter.peek() {
            if next_byte == 0x0020 || next_byte == 0x000D || next_byte == 0x0009
            || next_byte == 0x00A0 || next_byte == 0x000A || next_byte == 0x000C
            || next_byte == 0x0085 || next_byte == 0x000B {
                self.iter.next();
            } else {
                break;
            }
        }
    }

    ///
    /// Parses the `>` token that is expected to occur at the end of a opening tag.
    ///
    /// Returns `Option<GpxParserError` - As this parsing does not yield any data, the only possibility
    /// is an error if the expected next symbol encountered is not a `>` character.
    ///
    fn parse_lt_token(self) -> Option<GpxParserError> {
        if let Some(&Ok(next_byte)) = self.iter.peek() {
            if next_byte == 0x003C /* > */ {
                self.iter.next();
                None
            } else {
                Some(GpxParserError::ExpectedLessThenToken)
            }
        } else {
            Some(GpxParserError::UnexpectedEOF)
        }
    }

    ///
    ///
    ///
    fn parse_trkseg_tag_inner(self) {
        // Parse any whitespace until we find the first tag.
        self.parse_whitespace();
    }

    ///
    ///
    ///
    fn parse_trkpt_tag(self) {

    }

    ///
    ///
    ///
    fn parse_lat_attr(self) {

    }

    ///
    ///
    ///
    fn parse_lon_attr(self) {

    }

    ///
    ///
    ///
    fn parse_trkpt_tag_inner(self) {

    }

    ///
    ///
    ///
    fn parse_ele_tag(self) {

    }

    ///
    ///
    ///
    fn parse_ele_tag_inner(self) {

    }

    ///
    ///
    ///
    fn parse_time_tag(self) {

    }

    ///
    ///
    ///
    fn parse_time_tag_inner(self) {

    }
}
