use std::convert::{TryFrom, TryInto};

use crate::*;

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
    fn id(&self) -> &str {
        &self.id
    }
}

impl SwitchingSet for AudioSwitchingSet {
    type Track = AudioTrack;
    fn tracks(&self) -> &[AudioTrack] {
        &self.tracks
    }
    fn track(&self, id: &str) -> Option<&AudioTrack> {
        self.tracks.get(id)
    }
    fn tracks_mut(&mut self) -> &mut [AudioTrack] {
        &mut self.tracks
    }
    fn base_url(&self) -> &Option<RelativeBaseUrl> {
        &self.base_url
    }
    fn base_url_mut(&mut self) -> &mut Option<RelativeBaseUrl> {
        &mut self.base_url
    }
    fn mime_type(&self) -> &str {
        self.mime_type.as_ref()
    }
}

impl MediaSwitchingSet for AudioSwitchingSet {
    type MediaTrack = AudioTrack;
    const MEDIA_TYPE: MediaType = MediaType::Audio;
}

pub struct FrameRate(u64);

impl Default for FrameRate {
    fn default() -> Self {
        FrameRate(1024)
    }
}


impl TryFrom<AudioSwitchingSetDef> for AudioSwitchingSet {
    type Error = Error;
    fn try_from(def: AudioSwitchingSetDef) -> Result<Self> {
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
            sample_rate,
        } = def;
        let tracks = tracks
            .into_iter()
            .map(|track| {
                AudioTrack::new(
                    track,
                    codecs.as_ref(),
                    continuation_pattern.as_ref(),
                    frame_rate,
                    initialization_pattern.as_ref(),
                    media_time_offset,
                    sample_rate,
                )
            })
            .collect::<Result<Vec<AudioTrack>>>()?
            .try_into()?;
        Ok(AudioSwitchingSet {
            id,
            language,
            tracks,
            align_id,
            base_url,
            channels,
            label,
            mime_type,
            protection,
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
        let result = serde_json::from_str::<AudioSwitchingSet>(data);

        assert!(result.is_err());
        let error = result.unwrap_err().to_string();
        assert!(
            error.contains("must have codecs"),
            "Error did not indicate missing codecs `{}`",
            error
        );
    }
}
