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

pub type ExhibitionListExcelConfigData = Vec<ExhibitionListExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ExhibitionListExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "NEBALDLHNGL")]
    pub nebaldlhngl: i64,

    #[serde(rename = "MCPNIHGDENH")]
    pub mcpnihgdenh: i64,

    #[serde(rename = "BHKPDDKFGGO")]
    pub bhkpddkfggo: i64,

    #[serde(rename = "OAKCKMIBHKE")]
    pub oakckmibhke: i64,

    #[serde(rename = "displayType")]
    pub display_type: DisplayType,
}

#[derive(Serialize, Deserialize)]
pub enum DisplayType {
    #[serde(rename = "EXHIBITION_DISPLAY_TYPE_INT")]
    ExhibitionDisplayTypeInt,

    #[serde(rename = "EXHIBITION_DISPLAY_TYPE_TIME_MINSEC")]
    ExhibitionDisplayTypeTimeMinsec,
}
