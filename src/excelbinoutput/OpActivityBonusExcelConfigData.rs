// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_derive;

pub type OpActivityBonusExcelConfigData = Vec<OpActivityBonusExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct OpActivityBonusExcelConfigDatum {
    #[serde(rename = "bonusId")]
    pub bonus_id: i64,

    #[serde(rename = "sourceType")]
    pub source_type: SourceType,

    #[serde(rename = "sourceParam")]
    pub source_param: String,

    #[serde(rename = "openLevel")]
    pub open_level: i64,

    #[serde(rename = "bonusRatio")]
    pub bonus_ratio: i64,

    #[serde(rename = "textMapIdList")]
    pub text_map_id_list: Vec<String>,

    #[serde(rename = "trackPara")]
    pub track_para: Vec<Option<serde_json::Value>>,

    #[serde(rename = "CEJHCJGCNJF")]
    pub cejhcjgcnjf: i64,

    #[serde(rename = "FDCKJNNDHDM")]
    pub fdckjnndhdm: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum SourceType {
    #[serde(rename = "SOURCE_TYPE_BLOSSOM")]
    SourceTypeBlossom,

    #[serde(rename = "SOURCE_TYPE_DUNGEON")]
    SourceTypeDungeon,
}
