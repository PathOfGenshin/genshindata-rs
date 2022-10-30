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

pub type HomeWorldAreaComfortExcelConfigData = Vec<HomeWorldAreaComfortExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct HomeWorldAreaComfortExcelConfigDatum {
    #[serde(rename = "configID")]
    pub config_id: i64,

    #[serde(rename = "sceneID")]
    pub scene_id: i64,

    #[serde(rename = "areaID")]
    pub area_id: i64,

    #[serde(rename = "areaType")]
    pub area_type: Option<AreaType>,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "maxComfort")]
    pub max_comfort: i64,

    #[serde(rename = "PLPHFCALPGE")]
    pub plphfcalpge: String,

    #[serde(rename = "AMJBFLOINDL")]
    pub amjbfloindl: Amjbfloindl,

    #[serde(rename = "NABBOEKLIFC")]
    pub nabboeklifc: i64,

    #[serde(rename = "ANKKMCMGEEG")]
    pub ankkmcmgeeg: i64,

    #[serde(rename = "HAHOFNBLPNA")]
    pub hahofnblpna: i64,

    #[serde(rename = "AGNDDMFIICF")]
    pub agnddmfiicf: Vec<f64>,

    #[serde(rename = "CCNKJBLPDGN")]
    pub ccnkjblpdgn: Vec<f64>,
}

#[derive(Serialize, Deserialize)]
pub enum Amjbfloindl {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "-11.52,5.3,2.35")]
    The115253235,

    #[serde(rename = "-6.44,1.33,1.52")]
    The644133152,

    #[serde(rename = "6.57,3.47,14")]
    The65734714,
}

#[derive(Serialize, Deserialize)]
pub enum AreaType {
    #[serde(rename = "ExteriorArea")]
    ExteriorArea,
}
