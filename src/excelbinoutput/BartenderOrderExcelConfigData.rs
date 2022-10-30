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

pub type BartenderOrderExcelConfigData = Vec<BartenderOrderExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BartenderOrderExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "JIALCLKNJBI")]
    pub jialclknjbi: i64,

    #[serde(rename = "MCLAHLELLLD")]
    pub mclahlellld: Vec<i64>,

    #[serde(rename = "iconName")]
    pub icon_name: String,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "BACPJBHFCHJ")]
    pub bacpjbhfchj: Option<Bacpjbhfchj>,

    #[serde(rename = "time")]
    pub time: Option<i64>,

    #[serde(rename = "score")]
    pub score: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Bacpjbhfchj {
    #[serde(rename = "BARTENDER_CUP_BIG")]
    BartenderCupBig,

    #[serde(rename = "BARTENDER_CUP_MEDIUM")]
    BartenderCupMedium,

    #[serde(rename = "BARTENDER_CUP_SMALL")]
    BartenderCupSmall,
}
