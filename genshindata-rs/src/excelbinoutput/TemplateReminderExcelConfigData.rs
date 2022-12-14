// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type TemplateReminderExcelConfigData = Vec<TemplateReminderExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateReminderExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "icon")]
    pub icon: Icon,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "contentTextMapHash")]
    pub content_text_map_hash: i64,

    #[serde(rename = "param")]
    pub param: String,

    #[serde(rename = "showTime")]
    pub show_time: Option<f64>,

    #[serde(rename = "style")]
    pub style: Option<Style>,

    #[serde(rename = "activityType")]
    pub activity_type: Option<String>,

    #[serde(rename = "CNDKMFANBHH")]
    pub cndkmfanbhh: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Icon {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "UI_Tips_Item_ElemForce")]
    UiTipsItemElemForce,

    #[serde(rename = "UI_Tips_Item_Monster")]
    UiTipsItemMonster,

    #[serde(rename = "UI_Tips_Item_Warning")]
    UiTipsItemWarning,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Style {
    #[serde(rename = "DialogBox")]
    DialogBox,

    #[serde(rename = "MessageDialog")]
    MessageDialog,

    #[serde(rename = "MessagePushPageFirstTime")]
    MessagePushPageFirstTime,
}

pub fn load() -> Result<TemplateReminderExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "TemplateReminderExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
