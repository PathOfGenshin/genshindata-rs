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

pub type CusmtomGadgetSlotExcelConfigData = Vec<CusmtomGadgetSlotExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct CusmtomGadgetSlotExcelConfigDatum {
    #[serde(rename = "AHAHNFALNFN")]
    pub ahahnfalnfn: i64,

    #[serde(rename = "LAPGIHCIGHI")]
    pub lapgihcighi: Vec<i64>,

    #[serde(rename = "LGAGHGIGGJG")]
    pub lgaghgiggjg: Option<bool>,

    #[serde(rename = "HDPPKHCKFMM")]
    pub hdppkhckfmm: Vec<i64>,

    #[serde(rename = "NCAJMPOIPDB")]
    pub ncajmpoipdb: Vec<i64>,

    #[serde(rename = "IGLFOHCHEMP")]
    pub iglfohchemp: Option<i64>,
}
