use chrono::{DateTime, UTC};

pub struct GpxTrackPoint {
    lat: f32,
    lon: f32,
    elevation: f32,
    datetime: DateTime<UTC>
}
