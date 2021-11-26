use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use url::Url;

use crate::util::{Entity, RelativeUrl};
use crate::{
    normalize_tracks, AudioMimeType, AudioSwitchingSet, AudioTrack, Language, Number,
    SamplesPerFrame, ScaledDuration, ScaledValue, SegmentId, Segments, SwitchingSetProtection,
    TransferObjectIdentifierLimits,
};

#[skip_serializing_none]
#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AudioSwitchingSetData {
    pub id: String,
    pub language: Language,
    pub tracks: Vec<AudioTrackData>,
    pub align_id: Option<String>,
    #[serde(skip_serializing_if = "RelativeUrl::is_none")]
    pub base_url: RelativeUrl,
    pub channels: Option<u64>,
    pub codecs: Option<String>,
    pub continuation_pattern: Option<String>,
    pub samples_per_frame: Option<SamplesPerFrame>,
    pub initialization_pattern: Option<String>,
    pub label: Option<String>,
    pub media_time_offset: Option<ScaledValue>,
    pub mime_type: Option<AudioMimeType>,
    pub protection: Option<SwitchingSetProtection>,
    pub sample_rate: Option<u64>,
}

impl AudioSwitchingSetData {
    pub fn new(input: AudioSwitchingSet, location: &Url) -> Self {
        Self {
            id: input.id,
            language: input.language,
            tracks: input
                .tracks
                .into_iter()
                .map(|track| AudioTrackData::new(track, location))
                .collect(),
            align_id: input.align_id,
            base_url: RelativeUrl::None,
            channels: input.channels,
            codecs: None,
            continuation_pattern: None,
            samples_per_frame: None,
            initialization_pattern: None,
            label: input.label,
            media_time_offset: None,
            mime_type: Some(input.mime_type),
            protection: input.protection,
            sample_rate: None,
        }
    }

    pub fn normalize(&mut self) {
        normalize_tracks!(
            self,
            codecs,
            continuation_pattern,
            samples_per_frame,
            initialization_pattern,
            media_time_offset,
            sample_rate
        );
    }
}

#[skip_serializing_none]
#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AudioTrackData {
    pub id: String,
    pub bandwidth: Number,
    pub segments: Segments,
    #[serde(rename = "activeSegment")]
    pub active_segment_id: Option<SegmentId>,
    pub active_sequence_number: Option<u64>,
    pub average_bandwidth: Option<Number>,
    #[serde(skip_serializing_if = "RelativeUrl::is_none")]
    pub base_url: RelativeUrl,
    pub channels: Option<u64>,
    pub codecs: Option<String>,
    pub continuation_pattern: Option<String>,
    pub samples_per_frame: Option<SamplesPerFrame>,
    pub label: Option<String>,
    pub initialization_pattern: Option<String>,
    pub media_time_offset: Option<ScaledValue>,
    pub sample_rate: Option<u64>,
    pub segment_duration: Option<ScaledDuration>,
    pub toi_limits: Option<TransferObjectIdentifierLimits>,
}

impl AudioTrackData {
    pub fn new(input: AudioTrack, location: &Url) -> Self {
        let id = input.id().to_owned();
        Self {
            id,
            bandwidth: input.bandwidth,
            segments: input.segments,
            active_segment_id: input.active_segment_id,
            active_sequence_number: input.active_sequence_number,
            average_bandwidth: input.average_bandwidth,
            base_url: RelativeUrl::None,
            channels: input.channels,
            codecs: Some(input.codecs),
            continuation_pattern: Some(input.continuation_pattern.make_relative(location)),
            samples_per_frame: Some(input.samples_per_frame),
            label: input.label,
            initialization_pattern: Some(input.initialization_pattern.make_relative(location)),
            media_time_offset: Some(input.media_time_offset),
            sample_rate: Some(input.sample_rate),
            segment_duration: input.segment_duration,
            toi_limits: input.transmission.into(),
        }
    }
}

impl AudioTrackData {
    pub fn with_default_codecs(mut self, codecs: &Option<String>) -> Self {
        if self.codecs.is_none() {
            self.codecs = codecs.clone();
        }
        self
    }

    pub fn with_default_continuation_pattern(
        mut self,
        continuation_pattern: &Option<String>,
    ) -> Self {
        if self.continuation_pattern.is_none() {
            self.continuation_pattern = continuation_pattern.clone();
        }
        self
    }

    pub fn with_default_initialization_pattern(
        mut self,
        initialization_pattern: &Option<String>,
    ) -> Self {
        if self.initialization_pattern.is_none() {
            self.initialization_pattern = initialization_pattern.clone();
        }
        self
    }

    pub const fn with_default_samples_per_frame(
        mut self,
        samples_per_frame: Option<SamplesPerFrame>,
    ) -> Self {
        if self.samples_per_frame.is_none() {
            self.samples_per_frame = samples_per_frame;
        }
        self
    }

    pub const fn with_default_media_time_offset(
        mut self,
        media_time_offset: Option<ScaledValue>,
    ) -> Self {
        if self.media_time_offset.is_none() {
            self.media_time_offset = media_time_offset;
        }
        self
    }

    pub const fn with_default_sample_rate(mut self, sample_rate: Option<u64>) -> Self {
        if self.sample_rate.is_none() {
            self.sample_rate = sample_rate;
        }
        self
    }
}
