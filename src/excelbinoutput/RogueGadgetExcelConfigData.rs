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

pub type RogueGadgetExcelConfigData = Vec<RogueGadgetExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RogueGadgetExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "MKFOLLJINPJ")]
    pub mkfolljinpj: String,

    #[serde(rename = "gadgetId")]
    pub gadget_id: i64,

    #[serde(rename = "OFPABOGHPIG")]
    pub ofpaboghpig: Vec<Ofpaboghpig>,
}

#[derive(Serialize, Deserialize)]
pub struct Ofpaboghpig {
    #[serde(rename = "BHBCEHKMHJO")]
    pub bhbcehkmhjo: Option<String>,

    #[serde(rename = "NBNICLENPGL")]
    pub nbniclenpgl: Option<i64>,
}
