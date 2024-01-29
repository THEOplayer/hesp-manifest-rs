use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::util::{Entity, UInt, Uri};
use crate::{
    normalize_tracks, AudioMimeType, AudioSwitchingSet, AudioTrack, Language, SamplesPerFrame,
    ScaledDuration, ScaledValue, SegmentId, Segments, SwitchingSetProtection,
};

#[skip_serializing_none]
#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AudioSwitchingSetData {
    pub id: String,
    pub language: Language,
    pub tracks: Vec<AudioTrackData>,
    pub align_id: Option<String>,
    pub base_url: Option<Uri>,
    pub channels: Option<UInt>,
    pub codecs: Option<String>,
    pub continuation_pattern: Option<String>,
    pub samples_per_frame: Option<SamplesPerFrame>,
    pub initialization_pattern: Option<String>,
    pub label: Option<String>,
    pub media_time_offset: Option<ScaledValue>,
    pub mime_type: Option<AudioMimeType>,
    pub protection: Option<SwitchingSetProtection>,
    pub sample_rate: Option<UInt>,
}

impl From<AudioSwitchingSet> for AudioSwitchingSetData {
    fn from(input: AudioSwitchingSet) -> Self {
        Self {
            id: input.id().to_string(),
            language: input.language,
            tracks: input.tracks.into_iter().map(AudioTrackData::from).collect(),
            align_id: input.align_id.clone(),
            base_url: None,
            channels: input.channels.map(UInt::from),
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
}

impl AudioSwitchingSetData {
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
    pub bandwidth: UInt,
    pub segments: Segments,
    #[serde(default)]
    pub start_segment_id: SegmentId,
    #[serde(default)]
    pub start_sequence_number: UInt,
    pub average_bandwidth: Option<UInt>,
    pub base_url: Option<Uri>,
    pub channels: Option<UInt>,
    pub codecs: Option<String>,
    pub continuation_pattern: Option<String>,
    pub samples_per_frame: Option<SamplesPerFrame>,
    pub label: Option<String>,
    pub initialization_pattern: Option<String>,
    pub media_time_offset: Option<ScaledValue>,
    pub sample_rate: Option<UInt>,
    pub segment_duration: Option<ScaledDuration>,
}

impl From<AudioTrack> for AudioTrackData {
    fn from(input: AudioTrack) -> Self {
        let id = input.id().to_string();
        let (base_url, continuation_pattern, initialization_pattern) =
            if input.continuation_pattern.base_url() == input.initialization_pattern.base_url() {
                (
                    input.continuation_pattern.base_url().cloned(),
                    input.continuation_pattern.clone().into_pattern(),
                    input.initialization_pattern.clone().into_pattern(),
                )
            } else {
                (
                    None,
                    input.continuation_pattern.clone().into_full_pattern(),
                    input.initialization_pattern.clone().into_full_pattern(),
                )
            };
        Self {
            id,
            bandwidth: UInt::from(input.bandwidth),
            segments: input.segments,
            start_segment_id: SegmentId::default(),
            start_sequence_number: UInt::default(),
            average_bandwidth: input.average_bandwidth.map(UInt::from),
            base_url,
            channels: input.channels.map(UInt::from),
            codecs: Some(input.codecs),
            continuation_pattern: Some(continuation_pattern),
            samples_per_frame: Some(input.samples_per_frame),
            label: input.label,
            initialization_pattern: Some(initialization_pattern),
            media_time_offset: Some(input.media_time_offset),
            sample_rate: Some(input.sample_rate.into()),
            segment_duration: input.segment_duration,
        }
    }
}

impl AudioTrackData {
    #[must_use]
    pub fn with_default_codecs(mut self, codecs: &Option<String>) -> Self {
        if self.codecs.is_none() {
            self.codecs = codecs.clone();
        }
        self
    }

    #[must_use]
    pub fn with_default_continuation_pattern(
        mut self,
        continuation_pattern: &Option<String>,
    ) -> Self {
        if self.continuation_pattern.is_none() {
            self.continuation_pattern = continuation_pattern.clone();
        }
        self
    }

    #[must_use]
    pub fn with_default_initialization_pattern(
        mut self,
        initialization_pattern: &Option<String>,
    ) -> Self {
        if self.initialization_pattern.is_none() {
            self.initialization_pattern = initialization_pattern.clone();
        }
        self
    }

    #[must_use]
    pub const fn with_default_samples_per_frame(
        mut self,
        samples_per_frame: Option<SamplesPerFrame>,
    ) -> Self {
        if self.samples_per_frame.is_none() {
            self.samples_per_frame = samples_per_frame;
        }
        self
    }

    #[must_use]
    pub const fn with_default_media_time_offset(
        mut self,
        media_time_offset: Option<ScaledValue>,
    ) -> Self {
        if self.media_time_offset.is_none() {
            self.media_time_offset = media_time_offset;
        }
        self
    }

    #[must_use]
    pub const fn with_default_sample_rate(mut self, sample_rate: Option<UInt>) -> Self {
        if self.sample_rate.is_none() {
            self.sample_rate = sample_rate;
        }
        self
    }
}
