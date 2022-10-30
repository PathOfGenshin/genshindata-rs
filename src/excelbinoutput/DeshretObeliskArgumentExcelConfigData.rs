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

pub type DeshretObeliskArgumentExcelConfigData = Vec<DeshretObeliskArgumentExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct DeshretObeliskArgumentExcelConfigDatum {
    #[serde(rename = "MPPIJDJGHKL")]
    pub mppijdjghkl: i64,

    #[serde(rename = "groupIdList")]
    pub group_id_list: Vec<i64>,

    #[serde(rename = "KMKHMPANHNM")]
    pub kmkhmpanhnm: Vec<i64>,

    #[serde(rename = "effectName")]
    pub effect_name: EffectName,

    #[serde(rename = "LCPGABKHKLF")]
    pub lcpgabkhklf: Vec<f64>,

    #[serde(rename = "HMPCBLBOGCO")]
    pub hmpcblbogco: f64,

    #[serde(rename = "KPAMLCINPIF")]
    pub kpamlcinpif: f64,

    #[serde(rename = "MMKOGFKBHCP")]
    pub mmkogfkbhcp: Vec<f64>,

    #[serde(rename = "MGOANGLGJFP")]
    pub mgoanglgjfp: Vec<f64>,

    #[serde(rename = "HMKFPNCJDGH")]
    pub hmkfpncjdgh: f64,

    #[serde(rename = "KFOHMAHFGHB")]
    pub kfohmahfghb: f64,

    #[serde(rename = "NNLKFOKJCME")]
    pub nnlkfokjcme: f64,
}

#[derive(Serialize, Deserialize)]
pub enum EffectName {
    #[serde(rename = "Eff_SceneObj_DeshretObelisk_01_Search")]
    EffSceneObjDeshretObelisk01_Search,
}
