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

pub type MichiaeOverallExcelConfigData = Vec<MichiaeOverallExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MichiaeOverallExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "offeringId")]
    pub offering_id: i64,

    #[serde(rename = "CDKECIMACCK")]
    pub cdkecimacck: i64,

    #[serde(rename = "JEHGBBOGLIA")]
    pub jehgbboglia: i64,

    #[serde(rename = "EBEPENMFNKG")]
    pub ebepenmfnkg: Vec<i64>,

    #[serde(rename = "CGBPBOPMNLM")]
    pub cgbpbopmnlm: i64,

    #[serde(rename = "GNKOJJBABFE")]
    pub gnkojjbabfe: f64,

    #[serde(rename = "OMPLMPNPEMK")]
    pub omplmpnpemk: f64,

    #[serde(rename = "GNEMGJAAHCF")]
    pub gnemgjaahcf: i64,

    #[serde(rename = "NELCPCDAIBI")]
    pub nelcpcdaibi: f64,

    #[serde(rename = "KCKJIGDMJGK")]
    pub kckjigdmjgk: i64,

    #[serde(rename = "LEDJNJPMBIL")]
    pub ledjnjpmbil: i64,

    #[serde(rename = "BOAFLLHCOJI")]
    pub boafllhcoji: i64,
}
