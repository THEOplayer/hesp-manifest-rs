use thiserror::Error;
use crate::TransmissionType;

#[derive(Error, Debug)]
pub enum Error {
    #[error("activePresentation references unknown id {0}")]
    InvalidActivePresentationId(String),
    #[error("'{0}' is not a valid audio MIME Type")]
    InvalidAudioMime(String),
    #[error("'{0}' is not a valid video MIME Type")]
    InvalidVideoMime(String),
    #[error("Presentation with id {0} is active but has no currentTime")]
    MissingCurrentTime(String),
    #[error("segment id's must be incremented by one: {1} must not follow {0}")]
    InvalidSegmentIds(u64, u64),
    #[error("SwitchingSetProtection must contain at least one system")]
    EmptySwitchingSetProtectionSystems,
    #[error("TimeBounds must have a start- or end-time")]
    EmptyTimeBounds,
    #[error("startTime {start} must be before endTime {end}")]
    ReverseTimeBounds { start: u64, end: u64 },
    #[error("Track {0} has no segmentDuration therefore each segment must have timeBounds")]
    MissingSegmentDuration(String),
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
    #[error("Ids must be unique (duplicates: {0})")]
    DuplicateIds(String),
    #[error("ContinuationPattern '{0}' must contain {{segmentId}}")]
    InvalidContinuationPattern(String),
    #[error("InitializationPattern '{0}' must contain {{initId}}")]
    InvalidInitializationPattern(String),
    #[error("Presentation '{0}' must not contain multicast data")]
    InvalidUnicastPresentation(String),
    #[error("Presentation '{presentation}' is {transmission:?} therefore Track '{track}' must be {transmission:?}")]
    InvalidTrackTransmission { presentation:String, track:String, transmission:TransmissionType },
    #[error("Multicast presentation must have streamType 'live'")]
    InvalidMulticastStreamType,
    #[error("Track path '{0}' must contain exactly 4 forward slashes")]
    InvalidTrackPath(String),
    #[error("'{0}' is not a valid MediaType")]
    InvalidMediaType(String),
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
}

pub type Result<T> = std::result::Result<T, Error>;