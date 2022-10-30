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

pub type ActivitySumoStageExcelConfigData = Vec<ActivitySumoStageExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivitySumoStageExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "BOMLNJNENPD")]
    pub bomlnjnenpd: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,

    #[serde(rename = "OCNMJKJNEAL")]
    pub ocnmjkjneal: Vec<i64>,

    #[serde(rename = "IBPHNADLIKJ")]
    pub ibphnadlikj: Vec<i64>,

    #[serde(rename = "JLFCNBGMHJB")]
    pub jlfcnbgmhjb: Vec<i64>,

    #[serde(rename = "galleryId")]
    pub gallery_id: i64,

    #[serde(rename = "JMNODFCJGAC")]
    pub jmnodfcjgac: Vec<i64>,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "BPOCFHAPLFB")]
    pub bpocfhaplfb: Vec<i64>,

    #[serde(rename = "HBFMKDDMHBC")]
    pub hbfmkddmhbc: Vec<Hbfmkddmhbc>,

    #[serde(rename = "JNDEHIPPMJJ")]
    pub jndehippmjj: Vec<i64>,

    #[serde(rename = "IHDKIEDNPFK")]
    pub ihdkiednpfk: Vec<i64>,

    #[serde(rename = "BBFGIFIIFJJ")]
    pub bbfgifiifjj: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Hbfmkddmhbc {
    #[serde(rename = "BDCKBGFEHFA")]
    pub bdckbgfehfa: String,

    #[serde(rename = "BHKMMBBKGID")]
    pub bhkmmbbkgid: String,
}
