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

pub type SubQuestCatalogExcelConfigData = Vec<SubQuestCatalogExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct SubQuestCatalogExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "IKGENMHGIOE")]
    pub ikgenmhgioe: Ikgenmhgioe,

    #[serde(rename = "HCNONMEJIBF")]
    pub hcnonmejibf: Vec<Hcnonmejibf>,

    #[serde(rename = "KKCCBFMMDIP")]
    pub kkccbfmmdip: Vec<Hcnonmejibf>,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "LHMNPKCPCDH")]
    pub lhmnpkcpcdh: Option<Ikgenmhgioe>,
}

#[derive(Serialize, Deserialize)]
pub struct Hcnonmejibf {
    #[serde(rename = "type")]
    pub hcnonmejibf_type: Option<Type>,

    #[serde(rename = "param")]
    pub param: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "QUEST_CATALOG_COND_TYPE_QUEST")]
    QuestCatalogCondTypeQuest,
}

#[derive(Serialize, Deserialize)]
pub enum Ikgenmhgioe {
    #[serde(rename = "LOGIC_AND")]
    LogicAnd,

    #[serde(rename = "LOGIC_OR")]
    LogicOr,
}
