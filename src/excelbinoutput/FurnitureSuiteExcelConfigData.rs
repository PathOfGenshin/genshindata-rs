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

pub type FurnitureSuiteExcelConfigData = Vec<FurnitureSuiteExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct FurnitureSuiteExcelConfigDatum {
    #[serde(rename = "suiteID")]
    pub suite_id: i64,

    #[serde(rename = "jsonName")]
    pub json_name: String,

    #[serde(rename = "suiteNameTextMapHash")]
    pub suite_name_text_map_hash: i64,

    #[serde(rename = "DANHEFLIADG")]
    pub danhefliadg: i64,

    #[serde(rename = "favoriteNpcExcelIdVec")]
    pub favorite_npc_excel_id_vec: Vec<i64>,

    #[serde(rename = "GBDAPLKHCHE")]
    pub gbdaplkhche: String,

    #[serde(rename = "furnType")]
    pub furn_type: Vec<i64>,

    #[serde(rename = "itemIcon")]
    pub item_icon: String,

    #[serde(rename = "mapIcon")]
    pub map_icon: MapIcon,

    #[serde(rename = "ELHKGHAMBMG")]
    pub elhkghambmg: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub enum MapIcon {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "UI_MarkPoint_Homeworld_Suit")]
    UiMarkPointHomeworldSuit,
}
