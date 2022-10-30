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

pub type CoopPointExcelConfigData = Vec<CoopPointExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct CoopPointExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "chapterId")]
    pub chapter_id: i64,

    #[serde(rename = "type")]
    pub coop_point_excel_config_datum_type: Type,

    #[serde(rename = "acceptQuest")]
    pub accept_quest: Option<i64>,

    #[serde(rename = "postPointList")]
    pub post_point_list: Vec<i64>,

    #[serde(rename = "pointNameTextMapHash")]
    pub point_name_text_map_hash: i64,

    #[serde(rename = "pointDecTextMapHash")]
    pub point_dec_text_map_hash: i64,

    #[serde(rename = "pointPosId")]
    pub point_pos_id: i64,

    #[serde(rename = "photoMaleHashSuffix")]
    pub photo_male_hash_suffix: Option<i64>,

    #[serde(rename = "photoMaleHashPre")]
    pub photo_male_hash_pre: Option<i64>,

    #[serde(rename = "photoFemaleHashSuffix")]
    pub photo_female_hash_suffix: Option<i64>,

    #[serde(rename = "photoFemaleHashPre")]
    pub photo_female_hash_pre: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "POINT_END")]
    PointEnd,

    #[serde(rename = "POINT_MIDDLE")]
    PointMiddle,

    #[serde(rename = "POINT_START")]
    PointStart,
}
