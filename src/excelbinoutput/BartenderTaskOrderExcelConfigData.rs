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

pub type BartenderTaskOrderExcelConfigData = Vec<BartenderTaskOrderExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BartenderTaskOrderExcelConfigDatum {
    #[serde(rename = "questId")]
    pub quest_id: i64,

    #[serde(rename = "KGNOONGAMBK")]
    pub kgnoongambk: Vec<i64>,

    #[serde(rename = "CLHIEOCLBLH")]
    pub clhieoclblh: i64,

    #[serde(rename = "NHIOPIHEEEC")]
    pub nhiopiheeec: i64,

    #[serde(rename = "OKIABFOPABC")]
    pub okiabfopabc: i64,

    #[serde(rename = "HKAHLGLDJFJ")]
    pub hkahlgldjfj: Option<i64>,

    #[serde(rename = "LHHCCOGPFPN")]
    pub lhhccogpfpn: Option<bool>,
}
