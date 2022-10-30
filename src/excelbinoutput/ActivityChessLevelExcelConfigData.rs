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

pub type ActivityChessLevelExcelConfigData = Vec<ActivityChessLevelExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityChessLevelExcelConfigDatum {
    #[serde(rename = "level")]
    pub level: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "LEOMBPOJCDP")]
    pub leombpojcdp: Option<i64>,

    #[serde(rename = "NKBNCGPPIPH")]
    pub nkbncgppiph: i64,

    #[serde(rename = "HGKCGDOCOGF")]
    pub hgkcgdocogf: i64,

    #[serde(rename = "FIPNOCDEPBE")]
    pub fipnocdepbe: i64,

    #[serde(rename = "JCGINDOIJGO")]
    pub jcgindoijgo: i64,

    #[serde(rename = "OINKCCJEDBL")]
    pub oinkccjedbl: Vec<i64>,

    #[serde(rename = "INMEMDJAKPP")]
    pub inmemdjakpp: i64,

    #[serde(rename = "MKIFMNDAIFE")]
    pub mkifmndaife: Option<bool>,

    #[serde(rename = "PKGHHDNGPNC")]
    pub pkghhdngpnc: bool,

    #[serde(rename = "CGOCJKOCBAC")]
    pub cgocjkocbac: Option<String>,
}
