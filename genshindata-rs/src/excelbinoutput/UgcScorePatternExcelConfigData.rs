/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type UgcScorePatternExcelConfigData = Vec<UgcScorePatternExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct UgcScorePatternExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "scoreList")]
    pub score_list: Vec<String>,
    pub adhiniaofad: i64,
    pub chkhnlionhn: String,
    pub jhlgmgndhnc: Vec<i64>,
}
