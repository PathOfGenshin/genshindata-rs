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

pub type DisplayItemExcelConfigData = Vec<DisplayItemExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct DisplayItemExcelConfigDatum {
    #[serde(rename = "typeDescTextMapHash")]
    pub type_desc_text_map_hash: i64,

    #[serde(rename = "rankLevel")]
    pub rank_level: i64,

    #[serde(rename = "displayType")]
    pub display_type: Option<DisplayType>,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "itemType")]
    pub item_type: ItemType,

    #[serde(rename = "param")]
    pub param: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum DisplayType {
    #[serde(rename = "BARTENDER_ITEM")]
    BartenderItem,

    #[serde(rename = "DEFAULT_ITEM")]
    DefaultItem,
}

#[derive(Serialize, Deserialize)]
pub enum ItemType {
    #[serde(rename = "ITEM_DISPLAY")]
    ItemDisplay,
}
