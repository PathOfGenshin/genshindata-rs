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

pub type ActivityPhotographPosExcelConfigData = Vec<ActivityPhotographPosExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityPhotographPosExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "LDEDEHCFNOE")]
    pub ldedehcfnoe: i64,

    #[serde(rename = "JJNGBOEIBPL")]
    pub jjngboeibpl: i64,

    #[serde(rename = "NDGMEABLLDD")]
    pub ndgmeablldd: i64,

    #[serde(rename = "IIPHAFDCMBH")]
    pub iiphafdcmbh: i64,

    #[serde(rename = "IMIMFHBBILB")]
    pub imimfhbbilb: i64,

    #[serde(rename = "FHPPIPOEHCP")]
    pub fhppipoehcp: i64,

    #[serde(rename = "KLHHNJKMDDF")]
    pub klhhnjkmddf: String,

    #[serde(rename = "AMADEAJOCKF")]
    pub amadeajockf: String,

    #[serde(rename = "HMFFAIMDEOO")]
    pub hmffaimdeoo: i64,

    #[serde(rename = "AFBFFBMJBHI")]
    pub afbffbmjbhi: i64,

    #[serde(rename = "JKHOEGMCACD")]
    pub jkhoegmcacd: Vec<i64>,

    #[serde(rename = "GIELKPOFBFB")]
    pub gielkpofbfb: Vec<i64>,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "watcherId")]
    pub watcher_id: i64,

    #[serde(rename = "galleryId")]
    pub gallery_id: i64,

    #[serde(rename = "KBBHJNNAGDI")]
    pub kbbhjnnagdi: i64,
}
