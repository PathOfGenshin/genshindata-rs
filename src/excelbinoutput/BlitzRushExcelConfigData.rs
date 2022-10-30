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

pub type BlitzRushExcelConfigData = Vec<BlitzRushExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BlitzRushExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "contentDuration")]
    pub content_duration: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "LBOKGFIMAMN")]
    pub lbokgfimamn: i64,

    #[serde(rename = "BAPGILGMKDM")]
    pub bapgilgmkdm: i64,

    #[serde(rename = "MDGAHPBOKPK")]
    pub mdgahpbokpk: i64,

    #[serde(rename = "EBEDCJAHHFD")]
    pub ebedcjahhfd: Vec<i64>,

    #[serde(rename = "rewardPreview")]
    pub reward_preview: i64,
}
