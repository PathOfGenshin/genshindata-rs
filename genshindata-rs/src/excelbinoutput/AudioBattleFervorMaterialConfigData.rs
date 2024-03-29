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
    pub fhjegpeoolp: f64,
    pub bdfdenjaido: f64,
    pub mjkedbikean: i64,
    pub jeaibfhcpci: Vec<i64>,
    pub ggldppaghcl: Vec<Ggldppaghcl>,
    #[serde(rename = "type")]
    pub audio_battle_fervor_material_config_datum_type: Option<String>,
    pub bdlkccbakpd: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Ggldppaghcl {
    pub eblklhjdbgn: Option<String>,
}
