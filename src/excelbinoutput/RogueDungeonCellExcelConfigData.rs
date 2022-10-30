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

pub type RogueDungeonCellExcelConfigData = Vec<RogueDungeonCellExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RogueDungeonCellExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "IBKODJDNGMN")]
    pub ibkodjdngmn: i64,

    #[serde(rename = "DPKBHECFABD")]
    pub dpkbhecfabd: Vec<f64>,

    #[serde(rename = "IGJNJIFCBJO")]
    pub igjnjifcbjo: Vec<i64>,

    #[serde(rename = "FKLBMPJMBIL")]
    pub fklbmpjmbil: Vec<i64>,

    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "OMKPFNLDAFP")]
    pub omkpfnldafp: i64,

    #[serde(rename = "KOLDCFFPHGH")]
    pub koldcffphgh: Option<f64>,

    #[serde(rename = "KHDGAGPELFC")]
    pub khdgagpelfc: f64,

    #[serde(rename = "IDEHDOKNNND")]
    pub idehdoknnnd: f64,

    #[serde(rename = "FJMJMIGMJCL")]
    pub fjmjmigmjcl: Option<Fjmjmigmjcl>,

    #[serde(rename = "LGHJADBALHD")]
    pub lghjadbalhd: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub enum Fjmjmigmjcl {
    #[serde(rename = "ROGUE_CELL_TYPE_BOSS")]
    RogueCellTypeBoss,

    #[serde(rename = "ROGUE_CELL_TYPE_CHEST")]
    RogueCellTypeChest,

    #[serde(rename = "ROGUE_CELL_TYPE_INIT")]
    RogueCellTypeInit,
}
