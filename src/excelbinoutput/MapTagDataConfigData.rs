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

pub type MapTagDataConfigData = Vec<MapTagDataConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MapTagDataConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "type")]
    pub map_tag_data_config_datum_type: Option<String>,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "GIKOHDHDKBG")]
    pub gikohdhdkbg: i64,

    #[serde(rename = "MECOCMNPADF")]
    pub mecocmnpadf: i64,

    #[serde(rename = "icon")]
    pub icon: Icon,

    #[serde(rename = "hideBeforeUnlock")]
    pub hide_before_unlock: Option<bool>,

    #[serde(rename = "sceneIdList")]
    pub scene_id_list: Vec<i64>,

    #[serde(rename = "IODLIBKCLPK")]
    pub iodlibkclpk: Option<f64>,

    #[serde(rename = "HLALHJHGEAB")]
    pub hlalhjhgeab: Option<f64>,

    #[serde(rename = "unlockByDefault")]
    pub unlock_by_default: Option<bool>,

    #[serde(rename = "cityID")]
    pub city_id: Option<i64>,

    #[serde(rename = "LEJFLCPGKFE")]
    pub lejflcpgkfe: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Icon {
    #[serde(rename = "UI_MarkSwitch_Bigworld")]
    UiMarkSwitchBigworld,

    #[serde(rename = "UI_MarkSwitch_GoldenAppleIsles")]
    UiMarkSwitchGoldenAppleIsles,

    #[serde(rename = "UI_MarkSwitch_Homeworld")]
    UiMarkSwitchHomeworld,
}
