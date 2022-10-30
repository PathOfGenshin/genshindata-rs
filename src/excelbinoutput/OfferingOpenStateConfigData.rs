// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type OfferingOpenStateConfigData = Vec<OfferingOpenStateConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferingOpenStateConfigDatum {
    #[serde(rename = "offeringId")]
    pub offering_id: i64,

    #[serde(rename = "openState")]
    pub open_state: String,

    #[serde(rename = "itemLimit")]
    pub item_limit: String,

    #[serde(rename = "GLHGMGDLHEI")]
    pub glhgmgdlhei: String,

    #[serde(rename = "PCKMJCOKEOD")]
    pub pckmjcokeod: Option<bool>,

    #[serde(rename = "GDFHKAGNAEB")]
    pub gdfhkagnaeb: Option<bool>,

    #[serde(rename = "activityId")]
    pub activity_id: Option<i64>,
}

pub fn load() -> Result<OfferingOpenStateConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "OfferingOpenStateConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
