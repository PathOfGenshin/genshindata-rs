/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type EffigyChallengeV2TagExcelConfigData = Vec<EffigyChallengeV2TagExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EffigyChallengeV2TagExcelConfigDatum {
    pub id: i64,
    pub tag_desc_text_map_hash: i64,
    #[serde(rename = "FGFDKNIJGJG")]
    pub fgfdknijgjg: Vec<Vec<i64>>,
}
