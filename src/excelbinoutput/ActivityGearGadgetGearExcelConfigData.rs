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

pub type ActivityGearGadgetGearExcelConfigData = Vec<ActivityGearGadgetGearExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityGearGadgetGearExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "gadgetID")]
    pub gadget_id: i64,

    #[serde(rename = "EGINMJAFHCP")]
    pub eginmjafhcp: Vec<f64>,

    #[serde(rename = "IJJJLHGLNKE")]
    pub ijjjlhglnke: Vec<i64>,

    #[serde(rename = "materialID")]
    pub material_id: i64,

    #[serde(rename = "BPGCLMJOLGL")]
    pub bpgclmjolgl: String,

    #[serde(rename = "ANCLLCMHEGA")]
    pub ancllcmhega: String,

    #[serde(rename = "IPKALPKLINE")]
    pub ipkalpkline: String,
}