/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type UgcCustomLabelExcelConfigData = Vec<UgcCustomLabelExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct UgcCustomLabelExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    pub cdiamodgjcd: i64,
    pub dpikgecambd: Dpikgecambd,
    pub kcoepfggjdk: String,
    pub gojellojjlf: Vec<i64>,
    pub ajdgmmfbkkl: String,
    pub dhkajijihlm: String,
    pub ljkfgnngafa: String,
    pub ojfdlbonhkh: String,
    pub lndibjbceel: String,
    pub glekanimohm: String,
    #[serde(rename = "sortID")]
    pub sort_id: Option<i64>,
    pub khibaeomoin: String,
    pub hdmobeaefnn: i64,
    pub micfobkahbe: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Dpikgecambd {
    #[serde(rename = "UGC_DUNGEON_LABEL_BELONG_ADVANCE")]
    UgcDungeonLabelBelongAdvance,
    #[serde(rename = "UGC_DUNGEON_LABEL_BELONG_BASIC")]
    UgcDungeonLabelBelongBasic,
    #[serde(rename = "UGC_DUNGEON_LABEL_BELONG_NOT_SHOW")]
    UgcDungeonLabelBelongNotShow,
}
