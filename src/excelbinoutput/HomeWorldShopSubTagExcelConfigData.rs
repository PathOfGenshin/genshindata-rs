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

pub type HomeWorldShopSubTagExcelConfigData = Vec<HomeWorldShopSubTagExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct HomeWorldShopSubTagExcelConfigDatum {
    #[serde(rename = "subID")]
    pub sub_id: i64,

    #[serde(rename = "subTagTextMapHash")]
    pub sub_tag_text_map_hash: i64,

    #[serde(rename = "PAEBPDCLAAI")]
    pub paebpdclaai: Option<bool>,
}
