/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type LanV2ProjectionLevelExcelConfigData = Vec<LanV2ProjectionLevelExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct LanV2ProjectionLevelExcelConfigDatum {
    #[serde(rename = "levelId")]
    pub level_id: i64,
    #[serde(rename = "stageId")]
    pub stage_id: i64,
    #[serde(rename = "watcherId")]
    pub watcher_id: i64,
    #[serde(rename = "levelNameTextMapHash")]
    pub level_name_text_map_hash: i64,
    pub bobfheonbec: String,
    pub ikokgbfjccm: String,
    pub bplgkiddjjd: Option<f64>,
    pub cdafmpedcio: f64,
    pub comcmefeplp: f64,
    pub nkkgnfobedf: i64,
    pub emnfdpnfkdm: Vec<f64>,
    pub foccfjcpjnn: Vec<f64>,
    pub fgiklaacini: Vec<Option<serde_json::Value>>,
    pub hjigahhbjoo: Vec<i64>,
    pub lmlpifaikoa: Vec<i64>,
    pub ammebdhaicp: Vec<Vec<i64>>,
    pub jpaogipnhnd: Vec<Jpaogipnhnd>,
    pub nekhibipbac: Option<i64>,
    pub nopgcbmdlhf: Option<String>,
    pub chcicoilhnk: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Jpaogipnhnd {
    #[serde(rename = "prefabPath")]
    pub prefab_path: String,
    pub ilabgifkdda: String,
    pub ndioncoponc: Vec<f64>,
    pub pnklopgkjgf: Vec<f64>,
    pub enidlfclihf: Vec<i64>,
    pub amdehlohnpp: Vec<f64>,
    pub iphpgfeanki: Vec<f64>,
    pub gakhooppgcj: Option<i64>,
    pub edidohgkbid: Option<String>,
    pub bcliafdimlf: Option<f64>,
}
