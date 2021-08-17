use std::convert::{TryInto, TryFrom};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::*;

use super::VideoTrackDef;

#[skip_serializing_none]
#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(rename_all = "camelCase", try_from = "VideoSwitchingSetDef")]
pub struct VideoSwitchingSet {
    id: String,
    tracks: EntityVec<VideoTrack>,
    align_id: Option<String>,
    base_url: Option<RelativeBaseUrl>,
    label: Option<String>,
    mime_type: VideoMimeType,
    protection: Option<SwitchingSetProtection>,
}

impl Entity for VideoSwitchingSet {
    type Id = str;
    fn id(&self) -> &str { &self.id }
}

impl SwitchingSet for VideoSwitchingSet {
    type Track = VideoTrack;
    fn tracks(&self) -> &[VideoTrack] { &self.tracks }
    fn track(&self, id: &str) -> Option<&VideoTrack> { self.tracks.get(id) }
    fn tracks_mut(&mut self) -> &mut [VideoTrack] { &mut self.tracks }
    fn base_url(&self) -> &Option<RelativeBaseUrl> { &self.base_url }
    fn base_url_mut(&mut self) -> &mut Option<RelativeBaseUrl> {
        &mut self.base_url
    }
    fn mime_type(&self) -> &str { self.mime_type.as_ref() }
}

impl MediaSwitchingSet for VideoSwitchingSet {
    type MediaTrack = VideoTrack;
    const MEDIA_TYPE: MediaType = MediaType::Video;
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VideoSwitchingSetDef {
    id: String,
    tracks: EntityVec<VideoTrackDef>,
    align_id: Option<String>,
    base_url: Option<RelativeBaseUrl>,
    codecs: Option<String>,
    continuation_pattern: Option<ContinuationPattern>,
    frame_rate: Option<ScaledValue>,
    initialization_pattern: Option<InitializationPattern>,
    label: Option<String>,
    #[serde(default)]
    media_time_offset: ScaledValue,
    #[serde(default)]
    mime_type: VideoMimeType,
    protection: Option<SwitchingSetProtection>,
}

impl TryFrom<VideoSwitchingSetDef> for VideoSwitchingSet {
    type Error = Error;
    fn try_from(def: VideoSwitchingSetDef) -> Result<Self> {
        let VideoSwitchingSetDef {
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
            protection
        } = def;
        let tracks = tracks.into_iter().map(|track|
            VideoTrack::new(
                track,
                codecs.as_ref(),
                continuation_pattern.as_ref(),
                frame_rate,
                initialization_pattern.as_ref(),
                media_time_offset,
            )
        ).collect::<Result<Vec<VideoTrack>>>()?.try_into()?;
        Ok(VideoSwitchingSet { id, tracks, align_id, base_url, label, mime_type, protection })
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
