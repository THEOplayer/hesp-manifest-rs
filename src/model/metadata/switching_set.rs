use crate::data::MetadataSwitchingSetData;
use crate::util::{Entity, EntityIter, EntityIterMut, EntityMap, FromEntities};
use crate::{Address, Language, MediaType, MetadataTrack, Result, SwitchingSet};

#[derive(Clone, Debug)]
pub struct MetadataSwitchingSet {
    pub(crate) id: String,
    pub(crate) language: Option<Language>,
    pub(crate) tracks: EntityMap<MetadataTrack>,
    pub(crate) scheme_id: String,
    pub(crate) align_id: Option<String>,
    pub(crate) label: Option<String>,
    pub(crate) mime_type: String,
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

    fn track_mut(&mut self, id: &str) -> Option<&mut MetadataTrack> {
        self.tracks.get_mut(id)
    }

    fn tracks_mut(&mut self) -> EntityIterMut<MetadataTrack> {
        self.tracks.iter_mut()
    }

    fn mime_type(&self) -> &str {
        &self.mime_type
    }
}

impl MetadataSwitchingSet {
    pub fn new(
        presentation_id: &str,
        presentation_address: &Address,
        data: MetadataSwitchingSetData,
    ) -> Result<Self> {
        let address = presentation_address.join(data.base_url)?;
        let tracks = data
            .tracks
            .into_iter()
            .map(|track| {
                MetadataTrack::new(
                    presentation_id.to_owned(),
                    data.id.clone(),
                    &address,
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
