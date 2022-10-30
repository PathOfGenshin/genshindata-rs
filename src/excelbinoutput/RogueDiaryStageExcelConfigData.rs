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

pub type RogueDiaryStageExcelConfigData = Vec<RogueDiaryStageExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RogueDiaryStageExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "condId")]
    pub cond_id: i64,

    #[serde(rename = "NHAEJNNPBMH")]
    pub nhaejnnpbmh: Vec<i64>,

    #[serde(rename = "BLNKILNDHML")]
    pub blnkilndhml: Vec<i64>,

    #[serde(rename = "FOBNMOOCNFK")]
    pub fobnmoocnfk: Vec<i64>,

    #[serde(rename = "CFNNMONBCEJ")]
    pub cfnnmonbcej: i64,

    #[serde(rename = "JMLMNBOHOOG")]
    pub jmlmnbohoog: i64,

    #[serde(rename = "OFOHLHDCCPP")]
    pub ofohlhdccpp: String,

    #[serde(rename = "ANJJMPAMEKL")]
    pub anjjmpamekl: Option<i64>,

    #[serde(rename = "BNHKIIIIFMG")]
    pub bnhkiiiifmg: Vec<i64>,

    #[serde(rename = "ACNCOCPBLNF")]
    pub acncocpblnf: Vec<i64>,

    #[serde(rename = "watcherIdList")]
    pub watcher_id_list: Vec<i64>,

    #[serde(rename = "pushTipsId")]
    pub push_tips_id: i64,

    #[serde(rename = "DLGDLAIMIGL")]
    pub dlgdlaimigl: i64,

    #[serde(rename = "JPIKHPHDFAO")]
    pub jpikhphdfao: i64,

    #[serde(rename = "PCJBMLLPAMK")]
    pub pcjbmllpamk: Option<i64>,
}
