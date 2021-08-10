use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::*;

use super::AudioTrackDef;

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioSwitchingSet {
    id: String,
    language: Language,
    tracks: EntityVec<AudioTrack>,
    align_id: Option<String>,
    base_url: Option<RelativeBaseUrl>,
    channels: Option<u64>,
    label: Option<String>,
    mime_type: AudioMimeType,
    protection: Option<SwitchingSetProtection>,
}

impl Entity for AudioSwitchingSet {
    type Id = str;
    fn id(&self) -> &str { &self.id }
}

impl SwitchingSet for AudioSwitchingSet {
    type Track = AudioTrack;
    fn tracks(&self) -> &[Self::Track] { &self.tracks }
    fn base_url(&self) -> &Option<RelativeBaseUrl> { &self.base_url }
    fn mime_type(&self) -> &str { self.mime_type.as_ref() }
}

impl MediaSwitchingSet for AudioSwitchingSet {
    type MediaTrack = AudioTrack;
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AudioSwitchingSetDef {
    id: String,
    language: Language,
    tracks: EntityVec<AudioTrackDef>,
    align_id: Option<String>,
    base_url: Option<RelativeBaseUrl>,
    channels: Option<u64>,
    codecs: Option<String>,
    continuation_pattern: Option<ContinuationPattern>,
    #[serde(default = "AudioSwitchingSet::default_frame_rate")]
    frame_rate: u64,
    initialization_pattern: Option<InitializationPattern>,
    label: Option<String>,
    #[serde(default)]
    media_time_offset: ScaledValue,
    #[serde(default)]
    mime_type: AudioMimeType,
    protection: Option<SwitchingSetProtection>,
    sample_rate: Option<u64>,
}

impl AudioSwitchingSet {
    fn default_frame_rate() -> u64 { 1024 }

    fn new(def: AudioSwitchingSetDef) -> Result<Self> {
        let AudioSwitchingSetDef {
            id,
            language,
            tracks,
            align_id,
            base_url,
            channels,
            codecs,
            continuation_pattern,
            frame_rate,
            initialization_pattern,
            label,
            media_time_offset,
            mime_type,
            protection,
            sample_rate
        } = def;
        let tracks = tracks.into_iter().map(|track|
            AudioTrack::new(
                track,
                codecs.as_ref(),
                continuation_pattern.as_ref(),
                frame_rate,
                initialization_pattern.as_ref(),
                media_time_offset,
                sample_rate,
            )
        ).collect::<Result<Vec<AudioTrack>>>()?;
        Ok(AudioSwitchingSet {
            id,
            language,
            tracks: EntityVec::new(tracks)?,
            align_id,
            base_url,
            channels,
            label,
            mime_type,
            protection,
        })
    }
}

impl<'de> Deserialize<'de> for AudioSwitchingSet {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where D: serde::Deserializer<'de>
    {
        let def = AudioSwitchingSetDef::deserialize(deserializer)?;
        AudioSwitchingSet::new(def).map_err(serde::de::Error::custom)
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
        let result = serde_json::from_str::<AudioSwitchingSet>(data);

        assert!(result.is_err());
        let error = result.unwrap_err().to_string();
        assert!(
            error.contains("must have codecs"),
            "Error did not indicate missing codecs `{}`", error
        );
    }
}

