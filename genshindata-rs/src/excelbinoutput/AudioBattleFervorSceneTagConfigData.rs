/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type AudioBattleFervorSceneTagConfigData = Vec<AudioBattleFervorSceneTagConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioBattleFervorSceneTagConfigDatum {
    pub id: i64,
    pub area_id: i64,
    #[serde(rename = "LHFPLBHKDAP")]
    pub lhfplbhkdap: i64,
}
