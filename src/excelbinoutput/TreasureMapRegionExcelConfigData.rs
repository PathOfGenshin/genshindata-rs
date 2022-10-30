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

pub type TreasureMapRegionExcelConfigData = Vec<TreasureMapRegionExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct TreasureMapRegionExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "unlockDay")]
    pub unlock_day: i64,

    #[serde(rename = "tokenNum")]
    pub token_num: i64,

    #[serde(rename = "regionCenter")]
    pub region_center: Vec<f64>,

    #[serde(rename = "regionRadius")]
    pub region_radius: i64,

    #[serde(rename = "regionEntryId")]
    pub region_entry_id: i64,

    #[serde(rename = "groupList")]
    pub group_list: Vec<i64>,

    #[serde(rename = "reviseLevel")]
    pub revise_level: i64,

    #[serde(rename = "spotNumList")]
    pub spot_num_list: Vec<i64>,

    #[serde(rename = "tokenNumList")]
    pub token_num_list: Vec<i64>,

    #[serde(rename = "miscDropProbList")]
    pub misc_drop_prob_list: Vec<i64>,

    #[serde(rename = "miscDropId")]
    pub misc_drop_id: i64,

    #[serde(rename = "backupGroupList")]
    pub backup_group_list: Vec<i64>,

    #[serde(rename = "mpTokenThreshold")]
    pub mp_token_threshold: i64,

    #[serde(rename = "JFGPLAAPDPO")]
    pub jfgplaapdpo: Vec<f64>,

    #[serde(rename = "LJEDHBOODND")]
    pub ljedhboodnd: i64,

    #[serde(rename = "GNHCLEGFGBB")]
    pub gnhclegfgbb: Option<i64>,

    #[serde(rename = "mpGroupId")]
    pub mp_group_id: Option<i64>,

    #[serde(rename = "mpTypeId")]
    pub mp_type_id: Option<i64>,

    #[serde(rename = "OCFNEMPINGJ")]
    pub ocfnempingj: Option<i64>,
}
