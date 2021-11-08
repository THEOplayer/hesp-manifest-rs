use std::convert::{TryFrom, TryInto};

use crate::*;

pub struct MetadataSwitchingSet {
    id: String,
    language: Option<Language>,
    tracks: EntityVec<MetadataTrack>,
    scheme_id: String,
    align_id: Option<String>,
    base_url: Option<RelativeBaseUrl>,
    label: Option<String>,
    mime_type: String,
}

impl Entity for MetadataSwitchingSet {
    type Id = str;
    fn id(&self) -> &str {
        &self.id
    }
}

impl MetadataSwitchingSet {

}


impl TryFrom<MetadataSwitchingSetData> for MetadataSwitchingSet {
    type Error = Error;
    fn try_from(def: MetadataSwitchingSetData) -> Result<Self> {
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
        } = def;
        let tracks = tracks
            .into_iter()
            .map(|track| {
                MetadataTrack::new(track, continuation_pattern.as_ref(), media_time_offset)
            })
            .collect::<Result<Vec<MetadataTrack>>>()?
            .try_into()?;
        Ok(MetadataSwitchingSet {
            id,
            language,
            tracks,
            scheme_id,
            align_id,
            base_url,
            label,
            mime_type,
        })
    }
}
