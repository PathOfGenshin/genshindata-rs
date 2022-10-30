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

pub type SalvageChallengeDataExcelConfigData = Vec<SalvageChallengeDataExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct SalvageChallengeDataExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "playType")]
    pub play_type: String,

    #[serde(rename = "LDEDEHCFNOE")]
    pub ldedehcfnoe: i64,

    #[serde(rename = "galleryId")]
    pub gallery_id: Option<i64>,

    #[serde(rename = "IMFOHHHPNKJ")]
    pub imfohhhpnkj: Vec<i64>,

    #[serde(rename = "LAPGKLLKEIO")]
    pub lapgkllkeio: Vec<i64>,

    #[serde(rename = "HCADLIAKFGO")]
    pub hcadliakfgo: Option<i64>,

    #[serde(rename = "AOEBEJKFEBA")]
    pub aoebejkfeba: Vec<i64>,
}
