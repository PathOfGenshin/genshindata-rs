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

pub type TravelCatalogExcelConfigData = Vec<TravelCatalogExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct TravelCatalogExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "OOLPBKCAMFP")]
    pub oolpbkcamfp: i64,

    #[serde(rename = "questId")]
    pub quest_id: Option<i64>,

    #[serde(rename = "groupId")]
    pub group_id: Option<i64>,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "icon")]
    pub icon: String,
}
