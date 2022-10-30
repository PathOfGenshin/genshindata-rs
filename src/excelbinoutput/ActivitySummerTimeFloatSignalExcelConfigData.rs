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

pub type ActivitySummerTimeFloatSignalExcelConfigData =
    Vec<ActivitySummerTimeFloatSignalExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivitySummerTimeFloatSignalExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "CHOPKGHBDLF")]
    pub chopkghbdlf: Option<i64>,

    #[serde(rename = "KBLDPNPOLMB")]
    pub kbldpnpolmb: i64,

    #[serde(rename = "HCADLIAKFGO")]
    pub hcadliakfgo: i64,

    #[serde(rename = "GIBMNJLCBOB")]
    pub gibmnjlcbob: Option<bool>,
}
