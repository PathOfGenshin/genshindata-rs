/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivitySummerTimeRaceExcelConfigData = Vec<ActivitySummerTimeRaceExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActivitySummerTimeRaceExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "EHOPPOGGFII")]
    pub ehoppoggfii: Vec<i64>,
    #[serde(rename = "IKJHOCNGFNK")]
    pub ikjhocngfnk: Vec<i64>,
    #[serde(rename = "DBPDNALGCLJ")]
    pub dbpdnalgclj: Vec<i64>,
    #[serde(rename = "levelTitleTextMapHash")]
    pub level_title_text_map_hash: i64,
    #[serde(rename = "PABMLAFFDAL")]
    pub pabmlaffdal: i64,
    #[serde(rename = "BEAHEFGKLJL")]
    pub beahefgkljl: i64,
    #[serde(rename = "DDCJAFDLGKJ")]
    pub ddcjafdlgkj: i64,
    #[serde(rename = "HLGHMBFIDKI")]
    pub hlghmbfidki: i64,
    #[serde(rename = "IFKGOHBCMMF")]
    pub ifkgohbcmmf: i64,
}
