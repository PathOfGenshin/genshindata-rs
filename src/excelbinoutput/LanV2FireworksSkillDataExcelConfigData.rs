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

pub type LanV2FireworksSkillDataExcelConfigData = Vec<LanV2FireworksSkillDataExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct LanV2FireworksSkillDataExcelConfigDatum {
    #[serde(rename = "LPCEOEMLDBG")]
    pub lpceoemldbg: i64,

    #[serde(rename = "DCGGDMIPIAG")]
    pub dcggdmipiag: String,

    #[serde(rename = "HPMFPCLIFKF")]
    pub hpmfpclifkf: i64,

    #[serde(rename = "DDEFMMDJAKP")]
    pub ddefmmdjakp: i64,

    #[serde(rename = "PJCGIJICEAB")]
    pub pjcgijiceab: Option<i64>,

    #[serde(rename = "FJNIOCMFJAE")]
    pub fjniocmfjae: Option<i64>,

    #[serde(rename = "NNNMCDDDPEI")]
    pub nnnmcdddpei: Option<i64>,

    #[serde(rename = "LGFLPDDACPF")]
    pub lgflpddacpf: Vec<i64>,

    #[serde(rename = "HOPEMCLJNJN")]
    pub hopemcljnjn: i64,

    #[serde(rename = "JHLHNFNBMAK")]
    pub jhlhnfnbmak: i64,

    #[serde(rename = "PCIKFMDHEEG")]
    pub pcikfmdheeg: Vec<i64>,

    #[serde(rename = "NBGMKNCCLFD")]
    pub nbgmkncclfd: Option<i64>,
}
