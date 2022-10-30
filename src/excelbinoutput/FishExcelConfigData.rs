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

pub type FishExcelConfigData = Vec<FishExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct FishExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "monsterId")]
    pub monster_id: i64,

    #[serde(rename = "itemId")]
    pub item_id: i64,

    #[serde(rename = "hp")]
    pub hp: i64,

    #[serde(rename = "KFEILLAHGOK")]
    pub kfeillahgok: Vec<i64>,

    #[serde(rename = "FPBIBDDKFBH")]
    pub fpbibddkfbh: i64,

    #[serde(rename = "JNMEKDKFANI")]
    pub jnmekdkfani: i64,

    #[serde(rename = "BMKABBDJOPN")]
    pub bmkabbdjopn: Vec<f64>,

    #[serde(rename = "HNKHPIEJGDD")]
    pub hnkhpiejgdd: Vec<i64>,

    #[serde(rename = "PFPHEDFFBOI")]
    pub pfphedffboi: Vec<f64>,

    #[serde(rename = "DEMIKMGPKEE")]
    pub demikmgpkee: Vec<f64>,

    #[serde(rename = "IGOPPOMOKKC")]
    pub igoppomokkc: f64,

    #[serde(rename = "CMONLAAJHBC")]
    pub cmonlaajhbc: f64,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "FLJGHNBDNLF")]
    pub fljghnbdnlf: Vec<Option<serde_json::Value>>,

    #[serde(rename = "IFOMLHLEACL")]
    pub ifomlhleacl: i64,

    #[serde(rename = "HOOFABOAKPM")]
    pub hoofaboakpm: i64,

    #[serde(rename = "MACJIICKBBN")]
    pub macjiickbbn: Option<i64>,

    #[serde(rename = "AENFJNAKFFF")]
    pub aenfjnakfff: Option<i64>,
}
