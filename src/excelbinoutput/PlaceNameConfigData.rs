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

pub type PlaceNameConfigData = Vec<PlaceNameConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct PlaceNameConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "IPLGFOJOCEE")]
    pub iplgfojocee: Option<Iplgfojocee>,

    #[serde(rename = "ENGJBEACIGD")]
    pub engjbeacigd: String,

    #[serde(rename = "conditionType")]
    pub condition_type: ConditionType,

    #[serde(rename = "GLHENDJAKFH")]
    pub glhendjakfh: String,

    #[serde(rename = "NGBCEMLKLJC")]
    pub ngbcemlkljc: String,

    #[serde(rename = "actionType")]
    pub action_type: ActionType,

    #[serde(rename = "MPGMKJJPGPE")]
    pub mpgmkjjpgpe: i64,

    #[serde(rename = "type")]
    pub place_name_config_datum_type: Option<String>,

    #[serde(rename = "DLHPMOIOLFB")]
    pub dlhpmoiolfb: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum ActionType {
    #[serde(rename = "Close")]
    Close,

    #[serde(rename = "Open")]
    Open,

    #[serde(rename = "SwitchName")]
    SwitchName,
}

#[derive(Serialize, Deserialize)]
pub enum ConditionType {
    #[serde(rename = "ErosionArea")]
    ErosionArea,

    #[serde(rename = "MapArea")]
    MapArea,

    #[serde(rename = "Quest")]
    Quest,
}

#[derive(Serialize, Deserialize)]
pub enum Iplgfojocee {
    #[serde(rename = "Abyssalisle")]
    Abyssalisle,

    #[serde(rename = "MichiaeMatsuri")]
    MichiaeMatsuri,

    #[serde(rename = "TheChasm")]
    TheChasm,
}
