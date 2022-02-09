use crate::util::{Entity, EntityIter, EntityIterMut, EntityMap, FromEntities};
use crate::{
    Address, MediaType, Result, SwitchingSet, SwitchingSetProtection, ValidateSwitchingSet,
    VideoMimeType, VideoSwitchingSetData, VideoTrack,
};

#[derive(Debug, Clone)]
pub struct VideoSwitchingSet {
    pub(super) id: String,
    pub(super) tracks: EntityMap<VideoTrack>,
    pub(super) align_id: Option<String>,
    pub(super) label: Option<String>,
    pub(super) mime_type: VideoMimeType,
    pub(super) protection: Option<SwitchingSetProtection>,
}

impl VideoSwitchingSet {
    const MEDIA_TYPE: MediaType = MediaType::Video;
}

impl Entity for VideoSwitchingSet {
    fn id(&self) -> &str {
        &self.id
    }
}

impl SwitchingSet for VideoSwitchingSet {
    type Track = VideoTrack;

    fn media_type(&self) -> MediaType {
        Self::MEDIA_TYPE
    }

    fn tracks(&self) -> EntityIter<VideoTrack> {
        self.tracks.iter()
    }

    fn tracks_mut(&mut self) -> EntityIterMut<VideoTrack> {
        self.tracks.iter_mut()
    }

    fn track(&self, id: &str) -> Option<&VideoTrack> {
        self.tracks.get(id)
    }

    fn track_mut(&mut self, id: &str) -> Option<&mut VideoTrack> {
        self.tracks.get_mut(id)
    }

    fn mime_type(&self) -> &str {
        self.mime_type.as_ref()
    }
}

impl ValidateSwitchingSet<VideoTrack> for VideoSwitchingSet {}

impl VideoSwitchingSet {
    pub fn new(
        presentation_id: &str,
        presentation_address: &Address,
        data: VideoSwitchingSetData,
    ) -> Result<Self> {
        let address = presentation_address.join(data.base_url)?;
        let mime_type = data.mime_type.unwrap_or_default();
        let tracks = data
            .tracks
            .into_iter()
            .map(|track| {
                VideoTrack::new(
                    presentation_id.to_owned(),
                    data.id.clone(),
                    &address,
                    mime_type.clone(),
                    track
                        .with_default_codecs(&data.codecs)
                        .with_default_frame_rate(data.frame_rate)
                        .with_default_media_time_offset(data.media_time_offset)
                        .with_default_continuation_pattern(&data.continuation_pattern)
                        .with_default_initialization_pattern(&data.initialization_pattern),
                )
            })
            .into_entities()?;
        Ok(Self {
            id: data.id,
            tracks,
            align_id: data.align_id,
            label: data.label,
            mime_type,
            protection: data.protection,
        })
    }
}
