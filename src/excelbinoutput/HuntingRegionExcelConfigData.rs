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

pub type HuntingRegionExcelConfigData = Vec<HuntingRegionExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct HuntingRegionExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "centerPosList")]
    pub center_pos_list: Vec<f64>,

    #[serde(rename = "centerRadius")]
    pub center_radius: i64,

    #[serde(rename = "safeClueGroup")]
    pub safe_clue_group: Vec<i64>,

    #[serde(rename = "clueGroup")]
    pub clue_group: Vec<i64>,

    #[serde(rename = "safeDestinationGroup")]
    pub safe_destination_group: Vec<i64>,

    #[serde(rename = "destinationGroup")]
    pub destination_group: Vec<i64>,

    #[serde(rename = "regionInfoTextMapHash")]
    pub region_info_text_map_hash: i64,

    #[serde(rename = "AIHBPFCJKAG")]
    pub aihbpfcjkag: Vec<i64>,
}
