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

pub type OfferingLumenStoneExcelConfigData = Vec<OfferingLumenStoneExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct OfferingLumenStoneExcelConfigDatum {
    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "BGECDGPMJEP")]
    pub bgecdgpmjep: i64,

    #[serde(rename = "IDKFDAHEMLI")]
    pub idkfdahemli: i64,

    #[serde(rename = "iconPath")]
    pub icon_path: IconPath,

    #[serde(rename = "level")]
    pub level: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum IconPath {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "UI_ItemIcon_220048_02")]
    UiItemIcon220048_02,

    #[serde(rename = "UI_ItemIcon_220048_03")]
    UiItemIcon220048_03,
}
