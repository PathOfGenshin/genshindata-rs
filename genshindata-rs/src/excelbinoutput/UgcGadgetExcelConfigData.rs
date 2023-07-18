/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type UgcGadgetExcelConfigData = Vec<UgcGadgetExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct UgcGadgetExcelConfigDatum {
    pub jpfehlojgha: i64,
    pub nlkieiggnbi: String,
    pub ikccpokkedm: i64,
    #[serde(rename = "descriptionTextMapHash")]
    pub description_text_map_hash: i64,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "typeID")]
    pub type_id: i64,
    pub mhabpifdmcc: Option<String>,
    pub ficmiagcdop: Vec<i64>,
    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,
    pub dhpfhpjbpdk: f64,
    #[serde(rename = "cost")]
    pub cost: i64,
    pub pgomneecamf: Option<i64>,
    pub apdkfmioiak: Apdkfmioiak,
    pub pcmlaogfgco: String,
    pub ojkoofkhnlp: bool,
    pub jkeldijkbfh: i64,
    pub lnacdklielo: i64,
    pub cgamgmgfjik: Option<bool>,
    pub obbpofdkkmf: Option<bool>,
    pub fojccaeinpa: Option<bool>,
    pub onkoilaihil: Option<i64>,
    pub kapamhapchp: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Apdkfmioiak {
    #[serde(rename = "UGC_ROTATE_45")]
    UgcRotate45,
    #[serde(rename = "UGC_ROTATE_90")]
    UgcRotate90,
}
