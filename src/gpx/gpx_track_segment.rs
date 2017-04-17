use std::collections::BTreeMap;
use chrono::{DateTime, UTC};
use super::GpxTrackPoint;

pub struct GpxTrackSegment {
    start: Option<DateTime<UTC>>,
    end: Option<DateTime<UTC>>,
    points: Option<BTreeMap<DateTime<UTC>, GpxTrackPoint>>
}

impl GpxTrackSegment {
    pub fn new() -> GpxTrackSegment {
        GpxTrackSegment { start: None, end: None, points: None }
    }
}
