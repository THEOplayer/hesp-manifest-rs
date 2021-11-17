use url::Url;

use crate::util::{Entity, EntityMap, FromEntities, RelativeUrl};
use crate::*;

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

impl Entity for MetadataSwitchingSet {
    fn id(&self) -> &str {
        &self.id
    }
}

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
                    track
                        .with_default_continuation_pattern(&data.continuation_pattern)
                        .with_default_media_time_offset(data.media_time_offset),
                )
            })
            .into_entities()?;
        Ok(MetadataSwitchingSet {
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
