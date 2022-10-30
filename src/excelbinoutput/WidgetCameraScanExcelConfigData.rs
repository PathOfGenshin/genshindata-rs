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

pub type WidgetCameraScanExcelConfigData = Vec<WidgetCameraScanExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct WidgetCameraScanExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "cameraID")]
    pub camera_id: i64,

    #[serde(rename = "configID")]
    pub config_id: i64,

    #[serde(rename = "DEKBMLGHKOK")]
    pub dekbmlghkok: Vec<i64>,

    #[serde(rename = "EOHKKKPHDMJ")]
    pub eohkkkphdmj: bool,

    #[serde(rename = "DHLMOFLIAHL")]
    pub dhlmofliahl: String,
}
