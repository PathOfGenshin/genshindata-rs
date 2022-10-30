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

pub type QuestCatalogExcelConfigData = Vec<QuestCatalogExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct QuestCatalogExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "AIFAJKDPOPF")]
    pub aifajkdpopf: Vec<i64>,

    #[serde(rename = "questId")]
    pub quest_id: Vec<i64>,

    #[serde(rename = "DHBBCNBENDM")]
    pub dhbbcnbendm: Vec<Option<serde_json::Value>>,

    #[serde(rename = "BIFPAOIGKKH")]
    pub bifpaoigkkh: Vec<i64>,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "pic")]
    pub pic: String,
}
