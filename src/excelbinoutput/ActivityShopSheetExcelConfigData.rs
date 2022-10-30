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

pub type ActivityShopSheetExcelConfigData = Vec<ActivityShopSheetExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityShopSheetExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "isAheadPreview")]
    pub is_ahead_preview: Option<bool>,

    #[serde(rename = "SheetNameTextMapHash")]
    pub sheet_name_text_map_hash: i64,

    #[serde(rename = "cond")]
    pub cond: Vec<Cond>,

    #[serde(rename = "sortLevel")]
    pub sort_level: Vec<Option<serde_json::Value>>,
}

#[derive(Serialize, Deserialize)]
pub struct Cond {
    #[serde(rename = "param")]
    pub param: Vec<i64>,

    #[serde(rename = "type")]
    pub cond_type: Option<Type>,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ACTIVITY_SHOP_SHEET_COND_TIME_EQUAL_GREATER")]
    ActivityShopSheetCondTimeEqualGreater,
}
