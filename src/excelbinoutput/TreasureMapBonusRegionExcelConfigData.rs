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

pub type TreasureMapBonusRegionExcelConfigData = Vec<TreasureMapBonusRegionExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct TreasureMapBonusRegionExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "CBLDFOHGHLL")]
    pub cbldfohghll: i64,

    #[serde(rename = "reviseLevel")]
    pub revise_level: i64,

    #[serde(rename = "mapTitleTextMapHash")]
    pub map_title_text_map_hash: i64,

    #[serde(rename = "mapDescTextMapHash")]
    pub map_desc_text_map_hash: i64,

    #[serde(rename = "showImage")]
    pub show_image: String,

    #[serde(rename = "HAKNICGBHBB")]
    pub haknicgbhbb: i64,

    #[serde(rename = "regionCenter")]
    pub region_center: Vec<f64>,

    #[serde(rename = "regionRadius")]
    pub region_radius: i64,

    #[serde(rename = "unlockRegionId")]
    pub unlock_region_id: Option<i64>,
}
