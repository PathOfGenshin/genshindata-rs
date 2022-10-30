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

pub type ActivityVintageMarketPrepareExcelConfigData =
    Vec<ActivityVintageMarketPrepareExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityVintageMarketPrepareExcelConfigDatum {
    #[serde(rename = "configID")]
    pub config_id: i64,

    #[serde(rename = "CBOPDKGFCLD")]
    pub cbopdkgfcld: i64,

    #[serde(rename = "MODPLINEEJC")]
    pub modplineejc: Option<String>,

    #[serde(rename = "IBOFLEFMAEJ")]
    pub iboflefmaej: String,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "IFCABNCKGLN")]
    pub ifcabnckgln: i64,

    #[serde(rename = "CMPIGIDKDNJ")]
    pub cmpigidkdnj: i64,

    #[serde(rename = "JCGBOOHBEEO")]
    pub jcgboohbeeo: Option<i64>,

    #[serde(rename = "CNABJGNDDDI")]
    pub cnabjgndddi: Option<i64>,

    #[serde(rename = "OOELPGADFBE")]
    pub ooelpgadfbe: i64,

    #[serde(rename = "AOFANMFDGMK")]
    pub aofanmfdgmk: i64,
}
