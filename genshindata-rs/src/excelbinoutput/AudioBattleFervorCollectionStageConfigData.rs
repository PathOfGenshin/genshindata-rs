/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type AudioBattleFervorCollectionStageConfigData = Vec<AudioBattleFervorCollectionStageConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AudioBattleFervorCollectionStageConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    pub gbpmbmbldnk: f64,
    pub onhfddgogch: i64,
    pub ndpdgckgjnm: f64,
    pub ielpbdkbaok: Vec<Ielpbdkbaok>,
    pub pjpjlnihpgk: Vec<Pjpjlnihpgk>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Ielpbdkbaok {
    pub foefnedoakf: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Pjpjlnihpgk {
    pub llmaacchmmf: i64,
}
