use std::collections::BTreeMap;
use chrono::{DateTime, UTC};
use super::GpxTrackPoint;

pub struct GpxTrackSegment {
    start: DateTime<UTC>,
    end: DateTime<UTC>,
    points: BTreeMap<DateTime<UTC>, GpxTrackPoint>
}
