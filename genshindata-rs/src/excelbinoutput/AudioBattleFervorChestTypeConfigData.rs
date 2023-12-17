/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type AudioBattleFervorChestTypeConfigData = Vec<AudioBattleFervorChestTypeConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AudioBattleFervorChestTypeConfigDatum {
    pub gfjpencamid: i64,
    pub bglepkbbcah: f64,
    pub kkdieedmgmf: i64,
    pub ceacbnellml: i64,
    pub cpgjoaigdmm: i64,
    #[serde(rename = "type")]
    pub audio_battle_fervor_chest_type_config_datum_type: Option<String>,
}
