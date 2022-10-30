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

pub type CaptureExcelConfigData = Vec<CaptureExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct CaptureExcelConfigDatum {
    #[serde(rename = "monsterID")]
    pub monster_id: i64,

    #[serde(rename = "OIEPAMEMHNN")]
    pub oiepamemhnn: Vec<Oiepamemhnn>,
}

#[derive(Serialize, Deserialize)]
pub struct Oiepamemhnn {
    #[serde(rename = "AEOIJDKLDAO")]
    pub aeoijdkldao: Option<i64>,

    #[serde(rename = "dropID")]
    pub drop_id: Option<i64>,

    #[serde(rename = "itemID")]
    pub item_id: Option<i64>,
}
