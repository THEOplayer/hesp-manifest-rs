use std::convert::{TryFrom, TryInto};

use crate::*;
use crate::util::{Entity, EntityIter, EntityIterMut, EntityMap};

use super::data::VideoSwitchingSetData;

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

impl TryFrom<VideoSwitchingSetData> for VideoSwitchingSet {
    type Error = Error;
    fn try_from(def: VideoSwitchingSetData) -> Result<Self> {
        let VideoSwitchingSetData {
            id,
            tracks,
            align_id,
            base_url,
            codecs,
            continuation_pattern,
            frame_rate,
            initialization_pattern,
            label,
            media_time_offset,
            mime_type,
            protection,
        } = def;
        let tracks = tracks
            .into_iter()
            .map(|track| {
                VideoTrack::new(
                    track,
                    codecs.as_ref(),
                    continuation_pattern.as_ref(),
                    frame_rate,
                    initialization_pattern.as_ref(),
                    media_time_offset,
                )
            })
            .collect::<Result<Vec<VideoTrack>>>()?
            .try_into()?;
        Ok(VideoSwitchingSet {
            id,
            tracks,
            align_id,
            base_url,
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
        let value = serde_json::from_str::<VideoSwitchingSet>(data)?;
        serde_json::to_string(&value)?;
        Ok(())
    }
}
