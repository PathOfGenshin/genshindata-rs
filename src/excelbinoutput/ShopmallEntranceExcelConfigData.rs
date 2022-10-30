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

pub type ShopmallEntranceExcelConfigData = Vec<ShopmallEntranceExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ShopmallEntranceExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "subTabList")]
    pub sub_tab_list: Vec<i64>,

    #[serde(rename = "shopType")]
    pub shop_type: String,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "showType")]
    pub show_type: Option<String>,

    #[serde(rename = "order")]
    pub order: i64,
}
