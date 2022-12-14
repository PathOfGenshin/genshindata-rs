// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type NewActivityPushTipsConfigData = Vec<NewActivityPushTipsConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewActivityPushTipsConfigDatum {
    #[serde(rename = "pushTipsId")]
    pub push_tips_id: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "subtitleTextMapHash")]
    pub subtitle_text_map_hash: i64,

    #[serde(rename = "showIcon")]
    pub show_icon: Icon,

    #[serde(rename = "tabIcon")]
    pub tab_icon: Icon,

    #[serde(rename = "tutorialId")]
    pub tutorial_id: i64,

    #[serde(rename = "showImmediately")]
    pub show_immediately: Option<bool>,

    #[serde(rename = "activityId")]
    pub activity_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Icon {
    #[serde(rename = "UI_MessageIcon_Important")]
    UiMessageIconImportant,
}

pub fn load() -> Result<NewActivityPushTipsConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "NewActivityPushTipsConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
