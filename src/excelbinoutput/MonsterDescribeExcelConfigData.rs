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

pub type MonsterDescribeExcelConfigData = Vec<MonsterDescribeExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MonsterDescribeExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "titleID")]
    pub title_id: i64,

    #[serde(rename = "specialNameLabID")]
    pub special_name_lab_id: i64,

    #[serde(rename = "icon")]
    pub icon: String,
}
