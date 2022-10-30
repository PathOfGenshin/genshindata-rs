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

pub type GadgetGuestExcelConfigData = Vec<GadgetGuestExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct GadgetGuestExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "showType")]
    pub show_type: Option<ShowType>,
}

#[derive(Serialize, Deserialize)]
pub enum ShowType {
    #[serde(rename = "GRAP")]
    Grap,
}
