/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GcgCardViewExcelConfigData = Vec<GcgCardViewExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct GcgCardViewExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    pub icleekkkbfe: String,
    pub piadnamonhp: Vec<String>,
    pub jbkllgfcbgl: String,
    pub pinmjabaooi: String,
    pub ihbdlibphop: Vec<Option<serde_json::Value>>,
    pub lmcgdcdeekb: String,
    pub iljolocifcj: Iljolocifcj,
    pub ffanddimnne: Vec<String>,
    pub lfncjgofkak: Lfncjgofkak,
    pub nljfgodinod: Option<f64>,
    pub fpnpabbcofh: Option<f64>,
    pub dkmgjnilong: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Iljolocifcj {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "Gcg_CardFace_Char_Avatar_Cyno")]
    GcgCardFaceCharAvatarCyno,
    #[serde(rename = "Gcg_CardFace_Char_Avatar_Tartaglia_02")]
    GcgCardFaceCharAvatarTartaglia02,
    #[serde(rename = "Gcg_CardFace_Char_Avatar_Xiao")]
    GcgCardFaceCharAvatarXiao,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Lfncjgofkak {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "SkinEffectConfig_Character_Cyno2")]
    SkinEffectConfigCharacterCyno2,
    #[serde(rename = "SkinEffectConfig_Character_Tartaglia02")]
    SkinEffectConfigCharacterTartaglia02,
    #[serde(rename = "SkinEffectConfig_Character_Xiao2")]
    SkinEffectConfigCharacterXiao2,
}
