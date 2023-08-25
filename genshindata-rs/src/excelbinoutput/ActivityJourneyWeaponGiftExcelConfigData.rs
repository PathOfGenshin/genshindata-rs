/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityJourneyWeaponGiftExcelConfigData = Vec<ActivityJourneyWeaponGiftExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityJourneyWeaponGiftExcelConfigDatum {
    pub weapon_id: i64,
    #[serde(rename = "JOLIIGIGINE")]
    pub joliigigine: i64,
    pub icon_name: String,
}
