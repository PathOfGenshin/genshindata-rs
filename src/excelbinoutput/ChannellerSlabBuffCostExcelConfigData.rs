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

pub type ChannellerSlabBuffCostExcelConfigData = Vec<ChannellerSlabBuffCostExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ChannellerSlabBuffCostExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "buffQualityType")]
    pub buff_quality_type: String,

    #[serde(rename = "buffCost")]
    pub buff_cost: Option<i64>,

    #[serde(rename = "itemID")]
    pub item_id: Option<i64>,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,
}
