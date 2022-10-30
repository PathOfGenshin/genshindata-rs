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

pub type LampPhaseExcelConfigData = Vec<LampPhaseExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct LampPhaseExcelConfigDatum {
    #[serde(rename = "phaseId")]
    pub phase_id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "endProgress")]
    pub end_progress: Option<i64>,

    #[serde(rename = "materialVec")]
    pub material_vec: Vec<i64>,

    #[serde(rename = "GivingId")]
    pub giving_id: i64,

    #[serde(rename = "contribution")]
    pub contribution: i64,

    #[serde(rename = "addProgress")]
    pub add_progress: Option<i64>,

    #[serde(rename = "notifyGroupId")]
    pub notify_group_id: i64,

    #[serde(rename = "gadgetId")]
    pub gadget_id: Option<i64>,

    #[serde(rename = "isDisplay")]
    pub is_display: Option<bool>,
}
