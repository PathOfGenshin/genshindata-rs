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

pub type LunaRiteSearchingExcelConfigData = Vec<LunaRiteSearchingExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct LunaRiteSearchingExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "JDPGGCCINLP")]
    pub jdpggccinlp: String,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "IICLCFHFPPB")]
    pub iiclcfhfppb: i64,

    #[serde(rename = "LFHBNHPDLHD")]
    pub lfhbnhpdlhd: Vec<i64>,

    #[serde(rename = "regionCenter")]
    pub region_center: Vec<f64>,

    #[serde(rename = "HKEDHFCNEIH")]
    pub hkedhfcneih: i64,

    #[serde(rename = "NAHCOFHKGGL")]
    pub nahcofhkggl: i64,

    #[serde(rename = "regionRadius")]
    pub region_radius: f64,

    #[serde(rename = "HNJEDIOCHGB")]
    pub hnjediochgb: i64,

    #[serde(rename = "DGPLAEGCKPN")]
    pub dgplaegckpn: i64,

    #[serde(rename = "EFHHAPJKCPG")]
    pub efhhapjkcpg: i64,

    #[serde(rename = "AEGEKFJOKBD")]
    pub aegekfjokbd: i64,
}
