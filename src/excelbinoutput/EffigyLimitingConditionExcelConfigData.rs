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

pub type EffigyLimitingConditionExcelConfigData = Vec<EffigyLimitingConditionExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct EffigyLimitingConditionExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "conditionType")]
    pub condition_type: ConditionType,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "conditionParam1")]
    pub condition_param1: i64,

    #[serde(rename = "NGBCEMLKLJC")]
    pub ngbcemlkljc: Vec<i64>,

    #[serde(rename = "HGKDJMIAEBC")]
    pub hgkdjmiaebc: i64,

    #[serde(rename = "score")]
    pub score: i64,

    #[serde(rename = "exclusiveId")]
    pub exclusive_id: Option<i64>,

    #[serde(rename = "JFPFOFEGBOP")]
    pub jfpfofegbop: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub enum ConditionType {
    #[serde(rename = "EFFIGY_CONDITION_LEVEL_CONFIG")]
    EffigyConditionLevelConfig,

    #[serde(rename = "EFFIGY_CONDITION_MONSTER_CONFIG")]
    EffigyConditionMonsterConfig,

    #[serde(rename = "EFFIGY_CONDITION_TIME_LIMIT")]
    EffigyConditionTimeLimit,
}
