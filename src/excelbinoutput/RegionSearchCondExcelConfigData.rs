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

pub type RegionSearchCondExcelConfigData = Vec<RegionSearchCondExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RegionSearchCondExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "searchNameTextMapHash")]
    pub search_name_text_map_hash: i64,

    #[serde(rename = "searchDescTextMapHash")]
    pub search_desc_text_map_hash: i64,

    #[serde(rename = "searchMapDescTextMapHash")]
    pub search_map_desc_text_map_hash: i64,

    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "logicType")]
    pub logic_type: String,

    #[serde(rename = "cond")]
    pub cond: Vec<Cond>,

    #[serde(rename = "regionList")]
    pub region_list: Vec<i64>,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "totalProgress")]
    pub total_progress: i64,

    #[serde(rename = "reminderId")]
    pub reminder_id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Cond {
    #[serde(rename = "type")]
    pub cond_type: Option<String>,

    #[serde(rename = "param")]
    pub param: Vec<i64>,
}
