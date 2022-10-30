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

pub type ActivityPotionBuffExcelConfigData = Vec<ActivityPotionBuffExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityPotionBuffExcelConfigDatum {
    #[serde(rename = "buffId")]
    pub buff_id: i64,

    #[serde(rename = "GPDAEIMDCBF")]
    pub gpdaeimdcbf: String,

    #[serde(rename = "KCGENIAAGAN")]
    pub kcgeniaagan: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "descParam")]
    pub desc_param: Vec<String>,

    #[serde(rename = "GIHANMHEOGO")]
    pub gihanmheogo: i64,

    #[serde(rename = "MIKOFNEFALC")]
    pub mikofnefalc: i64,

    #[serde(rename = "KFGLFEAKNNM")]
    pub kfglfeaknnm: String,
}
