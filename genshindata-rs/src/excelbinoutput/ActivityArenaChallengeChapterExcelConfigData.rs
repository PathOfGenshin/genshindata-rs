// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ActivityArenaChallengeChapterExcelConfigData = Vec<ActivityArenaChallengeChapterExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityArenaChallengeChapterExcelConfigDatum {
    #[serde(rename = "chapterId")]
    pub chapter_id: i64,

    #[serde(rename = "FOKCEOIOIEA")]
    pub fokceoioiea: i64,

    #[serde(rename = "EMFIOJAPDLK")]
    pub emfiojapdlk: i64,

    #[serde(rename = "HNDONNCALIC")]
    pub hndonncalic: i64,

    #[serde(rename = "FHLAMHHAFHL")]
    pub fhlamhhafhl: i64,

    #[serde(rename = "CKEDNBOCALL")]
    pub ckednbocall: i64,
}

pub fn load() -> Result<ActivityArenaChallengeChapterExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivityArenaChallengeChapterExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
