/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type EffigyChallengeV2DifficultyExcelConfigData = Vec<EffigyChallengeV2DifficultyExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EffigyChallengeV2DifficultyExcelConfigDatum {
    #[serde(rename = "MIGOGNBFAFD")]
    pub migognbfafd: i64,
    pub id: Option<i64>,
    pub revise_level: Option<i64>,
}
