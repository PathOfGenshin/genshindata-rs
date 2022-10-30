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

pub type RefreshIndexExcelConfigData = Vec<RefreshIndexExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RefreshIndexExcelConfigDatum {
    #[serde(rename = "priority")]
    pub priority: i64,

    #[serde(rename = "type")]
    pub refresh_index_excel_config_datum_type: Option<Type>,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "refreshId")]
    pub refresh_id: i64,

    #[serde(rename = "rarity")]
    pub rarity: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "REFRESHINDEX_GADGET")]
    RefreshindexGadget,
}
