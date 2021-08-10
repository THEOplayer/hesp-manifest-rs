use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;

use crate::*;

use super::MetadataTrackDef;

#[skip_serializing_none]
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
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
    fn id(&self) -> &str { &self.id }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetadataSwitchingSetDef {
    id: String,
    mime_type: String,
    tracks: EntityVec<MetadataTrackDef>,
    scheme_id: String,
    align_id: Option<String>,
    base_url: Option<RelativeBaseUrl>,
    continuation_pattern: Option<ContinuationPattern>,
    label: Option<String>,
    language: Option<Language>,
    #[serde(default)]
    media_time_offset: ScaledValue,
}

impl MetadataSwitchingSet {
    fn new(def: MetadataSwitchingSetDef) -> Result<Self> {
        let MetadataSwitchingSetDef {
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
        let tracks = tracks.into_iter().map(|track|
            MetadataTrack::new(track, continuation_pattern.as_ref(), media_time_offset)
        ).collect::<Result<Vec<MetadataTrack>>>()?;
        Ok(MetadataSwitchingSet {
            id,
            language,
            tracks: EntityVec::new(tracks)?,
            scheme_id,
            align_id,
            base_url,
            label,
            mime_type,
        })
    }
}

impl<'de> Deserialize<'de> for MetadataSwitchingSet {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let def = MetadataSwitchingSetDef::deserialize(deserializer)?;
        MetadataSwitchingSet::new(def).map_err(serde::de::Error::custom)
    }
}
