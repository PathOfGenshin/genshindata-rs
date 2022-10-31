// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

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

    #[serde(rename = "PAALKBCMGLJ")]
    pub paalkbcmglj: String,

    #[serde(rename = "GIBKKIAKMHL")]
    pub gibkkiakmhl: String,

    #[serde(rename = "PCJEJMIPDLO")]
    pub pcjejmipdlo: Option<f64>,

    #[serde(rename = "GAOCAPACLOF")]
    pub gaocapaclof: f64,

    #[serde(rename = "KNEKHHLAJDD")]
    pub knekhhlajdd: f64,

    #[serde(rename = "NNFDMFHMGPG")]
    pub nnfdmfhmgpg: i64,

    #[serde(rename = "CNIPEIBOFKE")]
    pub cnipeibofke: Vec<f64>,

    #[serde(rename = "DFOGKDJHOPG")]
    pub dfogkdjhopg: Vec<f64>,

    #[serde(rename = "APHAGELGEOP")]
    pub aphagelgeop: Vec<Option<serde_json::Value>>,

    #[serde(rename = "NNDDICMNDOO")]
    pub nnddicmndoo: Vec<f64>,

    #[serde(rename = "PNKDOFIJOLK")]
    pub pnkdofijolk: Vec<f64>,

    #[serde(rename = "AAHBGJLIEFO")]
    pub aahbgjliefo: Vec<Vec<i64>>,

    #[serde(rename = "BHJALMDHIKL")]
    pub bhjalmdhikl: Vec<Bhjalmdhikl>,

    #[serde(rename = "FIMCKCLJJNE")]
    pub fimckcljjne: Option<i64>,

    #[serde(rename = "OFLJGHHFIDM")]
    pub ofljghhfidm: Option<String>,

    #[serde(rename = "PLDAJJGKMOJ")]
    pub pldajjgkmoj: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bhjalmdhikl {
    #[serde(rename = "prefabPath")]
    pub prefab_path: String,

    #[serde(rename = "FAELKCOMDCG")]
    pub faelkcomdcg: String,

    #[serde(rename = "MBDJDANJLAP")]
    pub mbdjdanjlap: Vec<f64>,

    #[serde(rename = "HBHPOEDJBHH")]
    pub hbhpoedjbhh: Vec<f64>,

    #[serde(rename = "OBKMFNHGDBD")]
    pub obkmfnhgdbd: Vec<f64>,

    #[serde(rename = "EILAPMHENNF")]
    pub eilapmhennf: Vec<f64>,

    #[serde(rename = "EKFHKKBKEIM")]
    pub ekfhkkbkeim: Vec<f64>,

    #[serde(rename = "CKOAPPANFMI")]
    pub ckoappanfmi: Option<i64>,

    #[serde(rename = "BBOHKNFOLAK")]
    pub bbohknfolak: Option<String>,

    #[serde(rename = "BGMMANAPOCI")]
    pub bgmmanapoci: Option<f64>,
}

pub fn load() -> Result<LanV2ProjectionLevelExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "LanV2ProjectionLevelExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}