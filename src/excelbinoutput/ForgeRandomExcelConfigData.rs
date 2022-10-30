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

pub type ForgeRandomExcelConfigData = Vec<ForgeRandomExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ForgeRandomExcelConfigDatum {
    #[serde(rename = "forgeRandomId")]
    pub forge_random_id: i64,

    #[serde(rename = "mainRandomItems")]
    pub main_random_items: Vec<MainRandomItem>,
}

#[derive(Serialize, Deserialize)]
pub struct MainRandomItem {
    #[serde(rename = "itemId")]
    pub item_id: Option<i64>,

    #[serde(rename = "count")]
    pub count: Option<i64>,

    #[serde(rename = "weight")]
    pub weight: Option<i64>,
}
