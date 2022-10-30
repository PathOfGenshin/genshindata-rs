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

pub type LanV2FireworksFactorDataExcelConfigData = Vec<LanV2FireworksFactorDataExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct LanV2FireworksFactorDataExcelConfigDatum {
    #[serde(rename = "LGDNPEHDOMO")]
    pub lgdnpehdomo: i64,

    #[serde(rename = "DENIPPFACIB")]
    pub denippfacib: Vec<i64>,

    #[serde(rename = "EIFFCHIDAOI")]
    pub eiffchidaoi: i64,

    #[serde(rename = "type")]
    pub lan_v2_fireworks_factor_data_excel_config_datum_type: String,

    #[serde(rename = "JJFBGMPGLBN")]
    pub jjfbgmpglbn: Vec<i64>,
}
