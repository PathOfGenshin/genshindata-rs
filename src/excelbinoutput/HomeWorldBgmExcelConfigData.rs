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

pub type HomeWorldBgmExcelConfigData = Vec<HomeWorldBgmExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct HomeWorldBgmExcelConfigDatum {
    #[serde(rename = "MJJENLEBKEF")]
    pub mjjenlebkef: i64,

    #[serde(rename = "NBIDHGOOCKD")]
    pub nbidhgoockd: bool,

    #[serde(rename = "JJMNJMCCOKP")]
    pub jjmnjmccokp: bool,

    #[serde(rename = "cityId")]
    pub city_id: i64,

    #[serde(rename = "sortOrder")]
    pub sort_order: i64,

    #[serde(rename = "GEGHMJBJMGB")]
    pub geghmjbjmgb: String,

    #[serde(rename = "LMLNBMJFFML")]
    pub lmlnbmjffml: i64,

    #[serde(rename = "MKBIFLBNLON")]
    pub mkbiflbnlon: Option<bool>,
}
