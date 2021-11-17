use url::Url;

use crate::util::{Entity, EntityIter, EntityIterMut, EntityMap, FromEntities, RelativeUrl};
use crate::*;

#[derive(Debug, Clone)]
pub struct VideoSwitchingSet {
    pub(super) id: String,
    pub(super) tracks: EntityMap<VideoTrack>,
    pub(super) align_id: Option<String>,
    pub(super) label: Option<String>,
    pub(super) mime_type: VideoMimeType,
    pub(super) protection: Option<SwitchingSetProtection>,
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
            mime_type: data.mime_type.unwrap_or_default(),
            protection: data.protection,
        })
    }
}