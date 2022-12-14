// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type GroupLinksBundleExcelConfigData = Vec<GroupLinksBundleExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupLinksBundleExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "groupList")]
    pub group_list: Vec<i64>,

    #[serde(rename = "IMFBJINHCDI")]
    pub imfbjinhcdi: i64,

    #[serde(rename = "EHMCNIDONMG")]
    pub ehmcnidonmg: Option<i64>,

    #[serde(rename = "rewardType")]
    pub reward_type: Option<RewardType>,

    #[serde(rename = "rewardId")]
    pub reward_id: Option<i64>,

    #[serde(rename = "reviseLevel")]
    pub revise_level: Option<i64>,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "GPLDOLGDACP")]
    pub gpldolgdacp: i64,

    #[serde(rename = "BKDHKMOKMFO")]
    pub bkdhkmokmfo: Option<i64>,

    #[serde(rename = "BFLHKOJKPKE")]
    pub bflhkojkpke: Option<bool>,

    #[serde(rename = "GAFKMPJCJND")]
    pub gafkmpjcjnd: Option<bool>,

    #[serde(rename = "CDAOPDKCMNB")]
    pub cdaopdkcmnb: Option<i64>,

    #[serde(rename = "playType")]
    pub play_type: Option<String>,

    #[serde(rename = "AKLMFDINDLG")]
    pub aklmfdindlg: Option<bool>,

    #[serde(rename = "OFGLJLJBFFB")]
    pub ofgljljbffb: Option<i64>,

    #[serde(rename = "IOAOGHGDDKD")]
    pub ioaoghgddkd: Option<bool>,

    #[serde(rename = "DHIJAMKBFGA")]
    pub dhijamkbfga: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RewardType {
    #[serde(rename = "FINISH")]
    Finish,
}

pub fn load() -> Result<GroupLinksBundleExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "GroupLinksBundleExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
