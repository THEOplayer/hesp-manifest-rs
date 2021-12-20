use url::Url;

use crate::util::{Entity, EntityIter, EntityIterMut, EntityMap, FromEntities};
use crate::{
    Language, MediaType, MetadataSwitchingSetData, MetadataTrack, Result, SwitchingSet,
    ValidateSwitchingSet,
};

#[derive(Clone, Debug)]
pub struct MetadataSwitchingSet {
    pub(super) id: String,
    pub(super) language: Option<Language>,
    pub(super) tracks: EntityMap<MetadataTrack>,
    pub(super) scheme_id: String,
    pub(super) align_id: Option<String>,
    pub(super) label: Option<String>,
    pub(super) mime_type: String,
}

impl MetadataSwitchingSet {
    const MEDIA_TYPE: MediaType = MediaType::Metadata;
}

impl Entity for MetadataSwitchingSet {
    fn id(&self) -> &str {
        &self.id
    }
}

impl SwitchingSet for MetadataSwitchingSet {
    type Track = MetadataTrack;

    fn media_type(&self) -> MediaType {
        Self::MEDIA_TYPE
    }

    fn tracks(&self) -> EntityIter<MetadataTrack> {
        self.tracks.iter()
    }

    fn track(&self, id: &str) -> Option<&MetadataTrack> {
        self.tracks.get(id)
    }

    fn tracks_mut(&mut self) -> EntityIterMut<MetadataTrack> {
        self.tracks.iter_mut()
    }

    fn mime_type(&self) -> &str {
        &self.mime_type
    }
}

impl ValidateSwitchingSet<MetadataTrack> for MetadataSwitchingSet {}

impl MetadataSwitchingSet {
    pub fn new(
        presentation_id: &str,
        presentation_url: &Url,
        data: MetadataSwitchingSetData,
    ) -> Result<Self> {
        let base_url = data.base_url.resolve(presentation_url)?;
        let tracks = data
            .tracks
            .into_iter()
            .map(|track| {
                MetadataTrack::new(
                    presentation_id.to_owned(),
                    data.id.clone(),
                    &base_url,
                    data.mime_type.clone(),
                    track
                        .with_default_codecs(&data.codecs)
                        .with_default_continuation_pattern(&data.continuation_pattern)
                        .with_default_media_time_offset(data.media_time_offset),
                )
            })
            .into_entities()?;
        Ok(Self {
            id: data.id,
            language: data.language,
            tracks,
            scheme_id: data.scheme_id,
            align_id: data.align_id,
            label: data.label,
            mime_type: data.mime_type,
        })
    }
}
