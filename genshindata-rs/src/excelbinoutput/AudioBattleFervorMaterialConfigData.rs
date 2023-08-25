/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type AudioBattleFervorMaterialConfigData = Vec<AudioBattleFervorMaterialConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AudioBattleFervorMaterialConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    pub cmhcobnkimj: f64,
    pub ihmgdahbpho: f64,
    pub kfhaobhcadi: i64,
    pub okbhfngkkig: Vec<i64>,
    pub ghplpnalijc: Vec<Ghplpnalijc>,
    #[serde(rename = "type")]
    pub audio_battle_fervor_material_config_datum_type: Option<String>,
    pub gmpedealnhh: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Ghplpnalijc {
    pub ngbhohpchia: Option<String>,
}
