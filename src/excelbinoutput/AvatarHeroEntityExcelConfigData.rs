// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type AvatarHeroEntityExcelConfigData = Vec<AvatarHeroEntityExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct AvatarHeroEntityExcelConfigDatum {
    #[serde(rename = "avatarId")]
    pub avatar_id: i64,

    #[serde(rename = "prefabPathHashSuffix")]
    pub prefab_path_hash_suffix: i64,

    #[serde(rename = "prefabPathHashPre")]
    pub prefab_path_hash_pre: i64,

    #[serde(rename = "gachaCardNameHashSuffix")]
    pub gacha_card_name_hash_suffix: i64,

    #[serde(rename = "coopPicNameHashSuffix")]
    pub coop_pic_name_hash_suffix: i64,
}

pub fn load() -> Result<AvatarHeroEntityExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "AvatarHeroEntityExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
