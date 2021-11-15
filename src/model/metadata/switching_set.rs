use url::Url;

use crate::util::{Entity, EntityMap, FromEntities, RelativeUrl};
use crate::*;

#[derive(Clone, Debug)]
pub struct MetadataSwitchingSet {
    id: String,
    language: Option<Language>,
    tracks: EntityMap<MetadataTrack>,
    scheme_id: String,
    align_id: Option<String>,
    label: Option<String>,
    mime_type: String,
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
        let MetadataSwitchingSetData {
            id,
            scheme_id,
            language,
            tracks,
            align_id,
            base_url,
            continuation_pattern,
            label,
            media_time_offset,
            mime_type,
        } = data;
        let base_url = base_url.resolve(presentation_url)?;
        let tracks = tracks
            .into_iter()
            .map(|track| {
                MetadataTrack::new(
                    presentation_id.to_owned(),
                    id.clone(),
                    &base_url,
                    track,
                    continuation_pattern.as_deref(),
                    media_time_offset,
                )
            })
            .into_entities()?;
        Ok(MetadataSwitchingSet {
            id,
            language,
            tracks,
            scheme_id,
            align_id,
            label,
            mime_type,
        })
    }
}
