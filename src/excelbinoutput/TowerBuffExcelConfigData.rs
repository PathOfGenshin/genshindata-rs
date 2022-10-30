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

pub type TowerBuffExcelConfigData = Vec<TowerBuffExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct TowerBuffExcelConfigDatum {
    #[serde(rename = "towerBuffId")]
    pub tower_buff_id: i64,

    #[serde(rename = "lastingType")]
    pub lasting_type: LastingType,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "buffIcon")]
    pub buff_icon: BuffIcon,

    #[serde(rename = "buffId")]
    pub buff_id: i64,
}

#[derive(Serialize, Deserialize)]
pub enum BuffIcon {
    #[serde(rename = "UI_Icon_TowerbeginCard_Attack")]
    UiIconTowerbeginCardAttack,

    #[serde(rename = "UI_Icon_TowerbeginCard_Defense")]
    UiIconTowerbeginCardDefense,

    #[serde(rename = "UI_Icon_TowerbeginCard_Life")]
    UiIconTowerbeginCardLife,
}

#[derive(Serialize, Deserialize)]
pub enum LastingType {
    #[serde(rename = "TOWER_BUFF_LASTING_FLOOR")]
    TowerBuffLastingFloor,

    #[serde(rename = "TOWER_BUFF_LASTING_IMMEDIATE")]
    TowerBuffLastingImmediate,

    #[serde(rename = "TOWER_BUFF_LASTING_LEVEL")]
    TowerBuffLastingLevel,
}
