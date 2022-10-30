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

pub type BlossomChestExcelConfigData = Vec<BlossomChestExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BlossomChestExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "chestGadgetId")]
    pub chest_gadget_id: i64,

    #[serde(rename = "worldResin")]
    pub world_resin: Option<i64>,

    #[serde(rename = "resin")]
    pub resin: Option<i64>,

    #[serde(rename = "refreshType")]
    pub refresh_type: String,

    #[serde(rename = "clientShowType")]
    pub client_show_type: Option<String>,
}
