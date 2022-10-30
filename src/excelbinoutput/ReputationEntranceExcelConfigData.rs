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

pub type ReputationEntranceExcelConfigData = Vec<ReputationEntranceExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ReputationEntranceExcelConfigDatum {
    #[serde(rename = "textId")]
    pub text_id: i64,

    #[serde(rename = "entranceId")]
    pub entrance_id: String,

    #[serde(rename = "cityId")]
    pub city_id: i64,

    #[serde(rename = "goodsIdVec")]
    pub goods_id_vec: Vec<GoodsIdVec>,

    #[serde(rename = "condNameVec")]
    pub cond_name_vec: Vec<i64>,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "explainTitleTextMapHash")]
    pub explain_title_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "iconName")]
    pub icon_name: String,

    #[serde(rename = "order")]
    pub order: i64,
}

#[derive(Serialize, Deserialize)]
pub struct GoodsIdVec {
    #[serde(rename = "type")]
    pub goods_id_vec_type: Option<Type>,

    #[serde(rename = "param1")]
    pub param1: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "REPUTATION_ENTRANCE_COND_QUEST")]
    ReputationEntranceCondQuest,

    #[serde(rename = "REPUTATION_ENTRANCE_COND_REPUTATION_LEVEL")]
    ReputationEntranceCondReputationLevel,
}
