// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ActivityPhotographExcelConfigData = Vec<ActivityPhotographExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityPhotographExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "CNDOICJDEOI")]
    pub cndoicjdeoi: Vec<i64>,

    #[serde(rename = "KDCMLONELPI")]
    pub kdcmlonelpi: Vec<i64>,

    #[serde(rename = "NCOPMPJAOJG")]
    pub ncopmpjaojg: f64,

    #[serde(rename = "JEBGIJFPFNG")]
    pub jebgijfpfng: f64,

    #[serde(rename = "MOEIJAIMDDK")]
    pub moeijaimddk: f64,

    #[serde(rename = "DGHOFCJBCEB")]
    pub dghofcjbceb: f64,

    #[serde(rename = "PLONGDIDOIO")]
    pub plongdidoio: f64,

    #[serde(rename = "ECOJIDDHMDN")]
    pub ecojiddhmdn: f64,

    #[serde(rename = "CFGOHJJAMFP")]
    pub cfgohjjamfp: f64,

    #[serde(rename = "JIAAOCDDJIJ")]
    pub jiaaocddjij: i64,

    #[serde(rename = "pushTipsID")]
    pub push_tips_id: i64,

    #[serde(rename = "APPCKKCMCBK")]
    pub appckkcmcbk: f64,

    #[serde(rename = "JDCOPGPHOAE")]
    pub jdcopgphoae: Vec<i64>,

    #[serde(rename = "KCCEPIDHGFO")]
    pub kccepidhgfo: Vec<i64>,

    #[serde(rename = "LLMLKCFMIBP")]
    pub llmlkcfmibp: Vec<i64>,
}

pub fn load() -> Result<ActivityPhotographExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivityPhotographExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
