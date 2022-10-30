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

pub type HomeWorldPlantExcelConfigData = Vec<HomeWorldPlantExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct HomeWorldPlantExcelConfigDatum {
    #[serde(rename = "NOLEBNOLICE")]
    pub nolebnolice: i64,

    #[serde(rename = "JDCAFOMGFKO")]
    pub jdcafomgfko: Vec<i64>,

    #[serde(rename = "CMDJNNPNOBM")]
    pub cmdjnnpnobm: Option<i64>,

    #[serde(rename = "CFALOPGHBNP")]
    pub cfalopghbnp: Option<i64>,

    #[serde(rename = "dropID")]
    pub drop_id: Option<i64>,

    #[serde(rename = "PCBGKJGGGJC")]
    pub pcbgkjgggjc: Vec<Pcbgkjgggjc>,

    #[serde(rename = "IDPDBNMBEJA")]
    pub idpdbnmbeja: i64,

    #[serde(rename = "time")]
    pub time: i64,

    #[serde(rename = "NHHFMNAMMEB")]
    pub nhhfmnammeb: i64,

    #[serde(rename = "DBKEIEDJJGP")]
    pub dbkeiedjjgp: i64,

    #[serde(rename = "BPAPEFCOLPH")]
    pub bpapefcolph: i64,

    #[serde(rename = "order")]
    pub order: i64,

    #[serde(rename = "inteeIconName")]
    pub intee_icon_name: InteeIconName,

    #[serde(rename = "GFIPOJOGCKD")]
    pub gfipojogckd: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Pcbgkjgggjc {
    #[serde(rename = "CMDJNNPNOBM")]
    pub cmdjnnpnobm: Option<i64>,

    #[serde(rename = "CFALOPGHBNP")]
    pub cfalopghbnp: Option<i64>,

    #[serde(rename = "dropID")]
    pub drop_id: Option<i64>,

    #[serde(rename = "weight")]
    pub weight: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum InteeIconName {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "UI_Icon_Item_GrowFlowers")]
    UiIconItemGrowFlowers,
}
