// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type LanV2ProjectionLevelExcelConfigData = Vec<LanV2ProjectionLevelExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LanV2ProjectionLevelExcelConfigDatum {
    #[serde(rename = "levelId")]
    pub level_id: i64,

    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "watcherId")]
    pub watcher_id: i64,

    #[serde(rename = "levelNameTextMapHash")]
    pub level_name_text_map_hash: i64,

    #[serde(rename = "ICPECCBDCGI")]
    pub icpeccbdcgi: String,

    #[serde(rename = "OPPIBPPFIAL")]
    pub oppibppfial: String,

    #[serde(rename = "OEOCGBCGIFE")]
    pub oeocgbcgife: Option<f64>,

    #[serde(rename = "MDKCODOKAKA")]
    pub mdkcodokaka: f64,

    #[serde(rename = "HKENJPJAACJ")]
    pub hkenjpjaacj: f64,

    #[serde(rename = "OGHDNHOBDJD")]
    pub oghdnhobdjd: i64,

    #[serde(rename = "NJPFPFDFOFD")]
    pub njpfpfdfofd: Vec<f64>,

    #[serde(rename = "GPABEJKNJLD")]
    pub gpabejknjld: Vec<f64>,

    #[serde(rename = "FOPBDONABPM")]
    pub fopbdonabpm: Vec<Option<serde_json::Value>>,

    #[serde(rename = "BEAAECFHDNH")]
    pub beaaecfhdnh: Vec<f64>,

    #[serde(rename = "FAAPNHFDFBJ")]
    pub faapnhfdfbj: Vec<f64>,

    #[serde(rename = "ELDKBANIKOI")]
    pub eldkbanikoi: Vec<Vec<i64>>,

    #[serde(rename = "GLBOGIBEJJE")]
    pub glbogibejje: Vec<Glbogibejje>,

    #[serde(rename = "LIJPIHOOANO")]
    pub lijpihooano: Option<i64>,

    #[serde(rename = "BNEHIGDCEJI")]
    pub bnehigdceji: Option<String>,

    #[serde(rename = "BENEALNGNOK")]
    pub benealngnok: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Glbogibejje {
    #[serde(rename = "prefabPath")]
    pub prefab_path: String,

    #[serde(rename = "EHMIOFHCKHK")]
    pub ehmiofhckhk: String,

    #[serde(rename = "OHHCAOGIELB")]
    pub ohhcaogielb: Vec<f64>,

    #[serde(rename = "ECIJFEFOLBH")]
    pub ecijfefolbh: Vec<f64>,

    #[serde(rename = "IICBPHACMDF")]
    pub iicbphacmdf: Vec<f64>,

    #[serde(rename = "KGCGPBMHGEB")]
    pub kgcgpbmhgeb: Vec<f64>,

    #[serde(rename = "DEPOJJNPJMA")]
    pub depojjnpjma: Vec<f64>,

    #[serde(rename = "ENMENFGLDDL")]
    pub enmenfglddl: Option<i64>,

    #[serde(rename = "INCNPFFFHIJ")]
    pub incnpfffhij: Option<String>,

    #[serde(rename = "KKMNECDJHFD")]
    pub kkmnecdjhfd: Option<f64>,
}

pub fn load() -> Result<LanV2ProjectionLevelExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "LanV2ProjectionLevelExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
