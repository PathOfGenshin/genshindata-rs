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

pub type RoguelikeRuneExcelConfigData = Vec<RoguelikeRuneExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RoguelikeRuneExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "MKBIFLBNLON")]
    pub mkbiflbnlon: bool,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "ILCKGIOMMEM")]
    pub ilckgiommem: String,

    #[serde(rename = "KLPCPLFJIJE")]
    pub klpcplfjije: String,

    #[serde(rename = "FOCKCPOHONB")]
    pub fockcpohonb: String,

    #[serde(rename = "BMNMCKOGIEE")]
    pub bmnmckogiee: String,

    #[serde(rename = "JDJAJMPAAPJ")]
    pub jdjajmpaapj: i64,

    #[serde(rename = "elementType")]
    pub element_type: i64,

    #[serde(rename = "abilityName")]
    pub ability_name: String,
}
