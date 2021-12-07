use serde::{Deserialize, Serialize};
use url::Url;

use crate::util::{Entity, EntityIter, EntityIterMut, EntityMap, FromEntities, UInt};
use crate::{
    AudioMimeType, AudioSwitchingSetData, AudioTrack, Language, Result, SwitchingSet,
    SwitchingSetProtection,
};

#[derive(Debug, Clone)]
pub struct AudioSwitchingSet {
    pub(super) id: String,
    pub(super) language: Language,
    pub(super) tracks: EntityMap<AudioTrack>,
    pub(super) align_id: Option<String>,
    pub(super) channels: Option<u64>,
    pub(super) label: Option<String>,
    pub(super) mime_type: AudioMimeType,
    pub(super) protection: Option<SwitchingSetProtection>,
}

impl Entity for AudioSwitchingSet {
    fn id(&self) -> &str {
        &self.id
    }
}

impl SwitchingSet for AudioSwitchingSet {
    type Track = AudioTrack;

    fn tracks(&self) -> EntityIter<AudioTrack> {
        self.tracks.iter()
    }
    fn track(&self, id: &str) -> Option<&AudioTrack> {
        self.tracks.get(id)
    }
    fn tracks_mut(&mut self) -> EntityIterMut<AudioTrack> {
        self.tracks.iter_mut()
    }
    fn mime_type(&self) -> &str {
        self.mime_type.as_ref()
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Deserialize, Serialize, Copy)]
pub struct SamplesPerFrame(#[serde(deserialize_with = "UInt::deserialize_u64")] u64);

impl Default for SamplesPerFrame {
    fn default() -> Self {
        Self(1024)
    }
}

impl AudioSwitchingSet {
    pub fn new(
        presentation_id: &str,
        presentation_url: &Url,
        data: AudioSwitchingSetData,
    ) -> Result<Self> {
        let base_url = data.base_url.resolve(presentation_url)?;
        let tracks = data
            .tracks
            .into_iter()
            .map(|track| {
                AudioTrack::new(
                    presentation_id.to_owned(),
                    data.id.clone(),
                    &base_url,
                    track
                        .with_default_sample_rate(data.sample_rate)
                        .with_default_codecs(&data.codecs)
                        .with_default_samples_per_frame(data.samples_per_frame)
                        .with_default_media_time_offset(data.media_time_offset)
                        .with_default_continuation_pattern(&data.continuation_pattern)
                        .with_default_initialization_pattern(&data.initialization_pattern),
                )
            })
            .into_entities()?;
        Ok(Self {
            id: data.id,
            language: data.language,
            tracks,
            align_id: data.align_id,
            channels: data.channels.map(u64::from),
            label: data.label,
            mime_type: data.mime_type.unwrap_or_default(),
            protection: data.protection,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codecs_in_set() {
        let data = r#"
            {
                "id": "0",
                "language": "aar",
                "tracks": [
                    {
                        "bandwidth": 1,
                        "id": "0",
                        "segments": []
                    }
                ]
            }"#;
        let url = Url::parse("https://www.theoplayer.com").unwrap();
        let data = serde_json::from_str::<AudioSwitchingSetData>(data).unwrap();
        let result = AudioSwitchingSet::new("p1", &url, data);

        assert!(result.is_err());
        let error = result.unwrap_err().to_string();
        assert!(
            error.contains("must have codecs"),
            "Error did not indicate missing codecs `{}`",
            error
        );
    }
}
