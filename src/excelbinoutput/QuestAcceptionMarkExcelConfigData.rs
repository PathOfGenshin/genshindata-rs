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

pub type QuestAcceptionMarkExcelConfigData = Vec<QuestAcceptionMarkExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct QuestAcceptionMarkExcelConfigDatum {
    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "PLNPGNLLBEM")]
    pub plnpgnllbem: i64,

    #[serde(rename = "npcId")]
    pub npc_id: Option<i64>,

    #[serde(rename = "IBGDMFJKGIB")]
    pub ibgdmfjkgib: Option<bool>,

    #[serde(rename = "sceneId")]
    pub scene_id: i64,

    #[serde(rename = "GAEFMOOGGAD")]
    pub gaefmooggad: Vec<f64>,

    #[serde(rename = "PPEDGAAJNEC")]
    pub ppedgaajnec: Option<bool>,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "PJBBGJJNGID")]
    pub pjbbgjjngid: i64,
}
