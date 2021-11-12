use url::Url;

use crate::util::{Entity, EntityIter, EntityIterMut, EntityMap};
use crate::*;

#[derive(Debug, Clone)]
pub struct AudioSwitchingSet {
    id: String,
    language: Language,
    tracks: EntityMap<AudioTrack>,
    align_id: Option<String>,
    channels: Option<u64>,
    label: Option<String>,
    mime_type: AudioMimeType,
    protection: Option<SwitchingSetProtection>,
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

impl AudioSwitchingSet {
    pub fn new(
        presentation_id: &str,
        presentation_url: &Url,
        data: AudioSwitchingSetData,
    ) -> Result<Self> {
        let AudioSwitchingSetData {
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
        } = data;
        let base_url = base_url.resolve(presentation_url)?;
        let tracks = tracks
            .into_iter()
            .map(|track| {
                AudioTrack::new(
                    presentation_id.to_owned(),
                    id.to_owned(),
                    &base_url,
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
