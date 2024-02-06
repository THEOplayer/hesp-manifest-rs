use std::num::ParseIntError;

use thiserror::Error;

use crate::SegmentId;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Scale must be strictly positive")]
    NullScale(),
    #[error("activePresentation references unknown id {0}")]
    InvalidActivePresentationId(String),
    #[error("'{0}' is not a valid audio MIME Type")]
    InvalidAudioMime(String),
    #[error("'{0}' is not a valid video MIME Type")]
    InvalidVideoMime(String),
    #[error("Presentation with id {0} is active but has no currentTime")]
    MissingCurrentTime(String),
    #[error("Presentation with id {0} is active but has no startTime")]
    MissingStartTime(String),
    #[error(
        "Presentation with id {0} is active but its currentTime is earlier than its startTime"
    )]
    ImpossibleCurrentTime(String),
    #[error("segment id's must be incremented by one: {1} must not follow {0}")]
    InvalidSegmentIds(SegmentId, SegmentId),
    #[error("SwitchingSetProtection must contain at least one system")]
    EmptySwitchingSetProtectionSystems,
    #[error("TimeBounds must have a start- or end-time")]
    EmptyTimeBounds,
    #[error("startTime {start} must be before endTime {end}")]
    ReverseTimeBounds { start: u64, end: u64 },
    #[error(
        "Track {0} has no segmentDuration therefore each segment must have timeBounds without gaps"
    )]
    MissingSegmentDuration(String),
    #[error("Track {0} is active so it must have an active sequence number")]
    MissingActiveSequenceNumber(String),
    #[error("Track {0} must have codecs")]
    MissingCodecs(String),
    #[error("Track {0} must have a continuation pattern")]
    MissingContinuationPattern(String),
    #[error("Track {0} must have a framerate")]
    MissingFrameRate(String),
    #[error("Track {0} must have a sample rate")]
    MissingSampleRate(String),
    #[error("Track {0} must have an initialization pattern")]
    MissingInitializationPattern(String),
    #[error("Ids must be unique (found duplicate: {0})")]
    DuplicateId(String),
    #[error("Pattern '{0}' must contain {1}")]
    InvalidPattern(String, &'static str),
    #[error("Track path '{0}' must contain exactly 3 forward slashes")]
    InvalidTrackPath(String),
    #[error("'{0}' is not a valid MediaType")]
    InvalidMediaType(String),
    #[error("'{0}' cannot be converted to a float (f64's mantissa is only 52 bits wide)")]
    FloatOverflow(String),
    #[error("Illegal key format version: {0}")]
    KeyFormatVersion(ParseIntError),
    #[error("Missing fairplay attribute: {0}")]
    MissingFairplayAttribute(&'static str),
    #[error("Invalid Fairplay scheme ID")]
    FairplaySchemeId,
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    #[error(transparent)]
    InvalidJson(#[from] serde_path_to_error::Error<serde_json::Error>),
}

pub type Result<T> = std::result::Result<T, Error>;
