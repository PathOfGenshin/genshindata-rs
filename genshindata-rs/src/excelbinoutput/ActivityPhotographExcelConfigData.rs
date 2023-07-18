/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityPhotographExcelConfigData = Vec<ActivityPhotographExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ActivityPhotographExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "activityId")]
    pub activity_id: i64,
    pub ddmgilidcfm: Vec<i64>,
    pub pnbggooiidp: Vec<i64>,
    pub ngifedlmmop: i64,
    pub phedgfmfecf: i64,
    pub apjlenpgdoo: i64,
    pub njafchemcfj: i64,
    pub hgdndfabfne: i64,
    pub eakegdmkmfp: i64,
    pub cmceggjddmj: i64,
    pub omginfcmdha: i64,
    #[serde(rename = "pushTipsID")]
    pub push_tips_id: i64,
    pub nheiiimmacp: i64,
    pub egkmcemhjbl: Vec<i64>,
    pub bcmghegbmfe: Vec<i64>,
    pub pljamicjjkh: Vec<i64>,
}
