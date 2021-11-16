use url::Url;

use crate::util::{Entity, EntityIter, EntityIterMut, EntityMap, FromEntities, RelativeUrl};
use crate::*;

#[derive(Debug, Clone)]
pub struct VideoSwitchingSet {
    id: String,
    tracks: EntityMap<VideoTrack>,
    align_id: Option<String>,
    label: Option<String>,
    mime_type: VideoMimeType,
    protection: Option<SwitchingSetProtection>,
}

impl Entity for VideoSwitchingSet {
    fn id(&self) -> &str {
        &self.id
    }
}

impl SwitchingSet for VideoSwitchingSet {
    type Track = VideoTrack;
    fn tracks(&self) -> EntityIter<VideoTrack> {
        self.tracks.iter()
    }
    fn track(&self, id: &str) -> Option<&VideoTrack> {
        self.tracks.get(id)
    }
    fn tracks_mut(&mut self) -> EntityIterMut<VideoTrack> {
        self.tracks.iter_mut()
    }
    fn mime_type(&self) -> &str {
        self.mime_type.as_ref()
    }
}

impl MediaSwitchingSet for VideoSwitchingSet {
    type MediaTrack = VideoTrack;
    const MEDIA_TYPE: MediaType = MediaType::Video;
}

impl VideoSwitchingSet {
    pub fn new(
        presentation_id: &str,
        presentation_url: &Url,
        data: VideoSwitchingSetData,
    ) -> Result<Self> {
        let base_url = data.base_url.resolve(presentation_url)?;
        let tracks = data
            .tracks
            .into_iter()
            .map(|track| {
                VideoTrack::new(
                    presentation_id.to_owned(),
                    data.id.clone(),
                    &base_url,
                    track
                        .with_default_codecs(&data.codecs)
                        .with_default_frame_rate(data.frame_rate)
                        .with_default_media_time_offset(data.media_time_offset)
                        .with_default_continuation_pattern(&data.continuation_pattern)
                        .with_default_initialization_pattern(&data.initialization_pattern),
                )
            })
            .into_entities()?;
        Ok(VideoSwitchingSet {
            id: data.id,
            tracks,
            align_id: data.align_id,
            label: data.label,
            mime_type: data.mime_type,
            protection: data.protection,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_and_serialize() -> anyhow::Result<()> {
        let data = r#"
          {
            "id": "main-video",
            "baseUrl": "video/",
            "frameRate": {
              "value": 25
            },
            "continuationPattern": "content-{segmentId}.mp4",
            "initializationPattern": "init-{initId}.mp4",
            "tracks": [
              {
                "id": "720p",
                "activeSegment": 1799,
                "activeSequenceNumber": 269999,
                "bandwidth": 3000000,
                "baseUrl": "720p/",
                "codecs": "avc1.4d001f",
                "resolution": {
                  "width": 1280,
                  "height": 720
                },
                "segmentDuration": {
                  "value": 540000,
                  "scale": 90000
                },
                "segments": [
                  {
                    "id": 1799,
                    "timeBounds": {
                      "startTime": 971460000,
                      "scale": 90000
                    }
                  }
                ]
              }
            ]
          }"#;
        //TODO
        // let value = serde_json::from_str::<VideoSwitchingSet>(data)?;
        // serde_json::to_string(&value)?;
        Ok(())
    }
}
