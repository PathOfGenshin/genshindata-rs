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

pub type LocalizationExcelConfigData = Vec<LocalizationExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct LocalizationExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "assetType")]
    pub asset_type: AssetType,

    #[serde(rename = "defaultPath")]
    pub default_path: String,

    #[serde(rename = "scPath")]
    pub sc_path: String,

    #[serde(rename = "tcPath")]
    pub tc_path: String,

    #[serde(rename = "enPath")]
    pub en_path: String,

    #[serde(rename = "krPath")]
    pub kr_path: String,

    #[serde(rename = "jpPath")]
    pub jp_path: String,

    #[serde(rename = "esPath")]
    pub es_path: String,

    #[serde(rename = "frPath")]
    pub fr_path: String,

    #[serde(rename = "idPath")]
    pub id_path: String,

    #[serde(rename = "ptPath")]
    pub pt_path: String,

    #[serde(rename = "ruPath")]
    pub ru_path: String,

    #[serde(rename = "thPath")]
    pub th_path: String,

    #[serde(rename = "viPath")]
    pub vi_path: String,

    #[serde(rename = "dePath")]
    pub de_path: String,

    #[serde(rename = "DKPALIBPOLC")]
    pub dkpalibpolc: String,

    #[serde(rename = "DIOGOEKBEDI")]
    pub diogoekbedi: String,
}

#[derive(Serialize, Deserialize)]
pub enum AssetType {
    #[serde(rename = "LOC_IMAGE")]
    LocImage,

    #[serde(rename = "LOC_SUBTITLE")]
    LocSubtitle,

    #[serde(rename = "LOC_TEXT")]
    LocText,

    #[serde(rename = "LOC_TROPHY_SET_ICON")]
    LocTrophySetIcon,
}
