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

pub type CombatEndCleanExcelConfigData = Vec<CombatEndCleanExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct CombatEndCleanExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "KGCBFNBIPHB")]
    pub kgcbfnbiphb: String,

    #[serde(rename = "BHHFPNOFCBA")]
    pub bhhfpnofcba: Vec<Bhhfpnofcba>,

    #[serde(rename = "EPOMPLNEHFC")]
    pub epomplnehfc: Vec<Epomplnehfc>,
}

#[derive(Serialize, Deserialize)]
pub enum Bhhfpnofcba {
    #[serde(rename = "Corruption")]
    Corruption,

    #[serde(rename = "None")]
    None,
}

#[derive(Serialize, Deserialize)]
pub enum Epomplnehfc {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "LevelEntity_ClearLocalGadgets")]
    LevelEntityClearLocalGadgets,
}
