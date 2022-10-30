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

pub type MechanicBuildingExcelConfigData = Vec<MechanicBuildingExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MechanicBuildingExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "gadgetId")]
    pub gadget_id: i64,

    #[serde(rename = "specialEffectLevel1")]
    pub special_effect_level1: Option<i64>,

    #[serde(rename = "specialEffectLevel2")]
    pub special_effect_level2: Option<i64>,

    #[serde(rename = "specialEffectDesc1TextMapHash")]
    pub special_effect_desc1_text_map_hash: i64,

    #[serde(rename = "specialEffectDesc2TextMapHash")]
    pub special_effect_desc2_text_map_hash: i64,

    #[serde(rename = "maxLevel")]
    pub max_level: i64,

    #[serde(rename = "openConds")]
    pub open_conds: Vec<OpenCond>,

    #[serde(rename = "defaultDungeonList")]
    pub default_dungeon_list: Vec<i64>,

    #[serde(rename = "elementType")]
    pub element_type: Option<i64>,

    #[serde(rename = "isEnableRotate")]
    pub is_enable_rotate: Option<bool>,

    #[serde(rename = "buildLimit")]
    pub build_limit: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct OpenCond {
    #[serde(rename = "key")]
    pub key: String,

    #[serde(rename = "value")]
    pub value: Option<f64>,
}
