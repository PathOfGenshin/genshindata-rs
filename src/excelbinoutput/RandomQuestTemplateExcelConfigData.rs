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

pub type RandomQuestTemplateExcelConfigData = Vec<RandomQuestTemplateExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RandomQuestTemplateExcelConfigDatum {
    #[serde(rename = "_mainId")]
    pub main_id: i64,

    #[serde(rename = "_elemList")]
    pub elem_list: Vec<ElemList>,
}

#[derive(Serialize, Deserialize)]
pub struct ElemList {
    #[serde(rename = "name")]
    pub name: Vec<String>,

    #[serde(rename = "poolId")]
    pub pool_id: Option<i64>,
}
