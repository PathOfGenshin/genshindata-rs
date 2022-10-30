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

pub type CompoundExcelConfigData = Vec<CompoundExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct CompoundExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "groupID")]
    pub group_id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "rankLevel")]
    pub rank_level: i64,

    #[serde(rename = "type")]
    pub compound_excel_config_datum_type: Type,

    #[serde(rename = "isDefaultUnlocked")]
    pub is_default_unlocked: Option<bool>,

    #[serde(rename = "costTime")]
    pub cost_time: i64,

    #[serde(rename = "queueSize")]
    pub queue_size: i64,

    #[serde(rename = "inputVec")]
    pub input_vec: Vec<PutVec>,

    #[serde(rename = "outputVec")]
    pub output_vec: Vec<PutVec>,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "MMBLAPAKLIC")]
    pub mmblapaklic: i64,
}

#[derive(Serialize, Deserialize)]
pub struct PutVec {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "count")]
    pub count: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "COMPOUND_COOK")]
    CompoundCook,
}
