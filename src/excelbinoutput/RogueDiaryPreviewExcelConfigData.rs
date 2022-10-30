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

pub type RogueDiaryPreviewExcelConfigData = Vec<RogueDiaryPreviewExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RogueDiaryPreviewExcelConfigDatum {
    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "LBOKGFIMAMN")]
    pub lbokgfimamn: i64,

    #[serde(rename = "GDGCNJNBLHB")]
    pub gdgcnjnblhb: i64,

    #[serde(rename = "OOIFPKDKJBE")]
    pub ooifpkdkjbe: Vec<i64>,

    #[serde(rename = "CPFPEPGDFGM")]
    pub cpfpepgdfgm: Vec<i64>,

    #[serde(rename = "OEGMFOHBAHC")]
    pub oegmfohbahc: Vec<i64>,

    #[serde(rename = "EBLDEKEMJLO")]
    pub ebldekemjlo: i64,

    #[serde(rename = "BLNOPOOOJFB")]
    pub blnopooojfb: i64,

    #[serde(rename = "worldSceneId")]
    pub world_scene_id: i64,
}
