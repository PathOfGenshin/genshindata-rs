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

pub type BirthdayMailExcelConfigData = Vec<BirthdayMailExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BirthdayMailExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "mailId")]
    pub mail_id: i64,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "effectiveDate")]
    pub effective_date: String,

    #[serde(rename = "JDNIFLEAPHB")]
    pub jdnifleaphb: i64,
}
