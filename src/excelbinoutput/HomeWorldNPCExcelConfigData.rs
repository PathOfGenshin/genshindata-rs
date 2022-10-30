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

pub type HomeWorldNpcExcelConfigData = Vec<HomeWorldNpcExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct HomeWorldNpcExcelConfigDatum {
    #[serde(rename = "furnitureID")]
    pub furniture_id: i64,

    #[serde(rename = "avatarID")]
    pub avatar_id: Option<i64>,

    #[serde(rename = "MECLDFABFMF")]
    pub mecldfabfmf: i64,

    #[serde(rename = "AAIIGOGAJOL")]
    pub aaiigogajol: Vec<i64>,

    #[serde(rename = "AAIAIEHDDHM")]
    pub aaiaiehddhm: Aaiaiehddhm,

    #[serde(rename = "LOIHCKNLOKB")]
    pub loihcknlokb: Loihcknlokb,

    #[serde(rename = "GMPOLDOJIPF")]
    pub gmpoldojipf: Aaiaiehddhm,

    #[serde(rename = "DBGNKJKLOLL")]
    pub dbgnkjkloll: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "PBMEJEABAKN")]
    pub pbmejeabakn: Option<bool>,

    #[serde(rename = "EPLACNGABCG")]
    pub eplacngabcg: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum Aaiaiehddhm {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "UI_AvatarIcon_Side_Paimon")]
    UiAvatarIconSidePaimon,
}

#[derive(Serialize, Deserialize)]
pub enum Loihcknlokb {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "UI_AvatarIcon_Paimon")]
    UiAvatarIconPaimon,
}
