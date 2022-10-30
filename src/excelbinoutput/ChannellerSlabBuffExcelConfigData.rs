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

pub type ChannellerSlabBuffExcelConfigData = Vec<ChannellerSlabBuffExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ChannellerSlabBuffExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "buffNameTextMapHash")]
    pub buff_name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "descParam")]
    pub desc_param: Vec<String>,

    #[serde(rename = "materialID")]
    pub material_id: i64,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "buffQualityType")]
    pub buff_quality_type: BuffQualityType,
}

#[derive(Serialize, Deserialize)]
pub enum BuffQualityType {
    #[serde(rename = "QUALITY_BLUE")]
    QualityBlue,

    #[serde(rename = "QUALITY_GREEN")]
    QualityGreen,

    #[serde(rename = "QUALITY_PURPLE")]
    QualityPurple,
}