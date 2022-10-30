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

pub type CustomLevelComponentConfigData = Vec<CustomLevelComponentConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct CustomLevelComponentConfigDatum {
    #[serde(rename = "MPKOBAJFKAD")]
    pub mpkobajfkad: i64,

    #[serde(rename = "typeID")]
    pub type_id: i64,

    #[serde(rename = "HPKJLBKIFFN")]
    pub hpkjlbkiffn: i64,

    #[serde(rename = "GALOMNANAPD")]
    pub galomnanapd: i64,

    #[serde(rename = "KKOFIKGCDJH")]
    pub kkofikgcdjh: Option<i64>,

    #[serde(rename = "CEGDPPGKIBC")]
    pub cegdppgkibc: Option<i64>,

    #[serde(rename = "NKBBAKKGLIG")]
    pub nkbbakkglig: String,

    #[serde(rename = "IDNNHDDHCPN")]
    pub idnnhddhcpn: i64,

    #[serde(rename = "OOBMAPMMDAP")]
    pub oobmapmmdap: i64,

    #[serde(rename = "FADLNCMBMBC")]
    pub fadlncmbmbc: i64,

    #[serde(rename = "OMDHKAJHHLK")]
    pub omdhkajhhlk: Option<Omdhkajhhlk>,

    #[serde(rename = "IMJOKCKCLCK")]
    pub imjokckclck: Option<i64>,

    #[serde(rename = "IMMPEHFFMBD")]
    pub immpehffmbd: Option<bool>,

    #[serde(rename = "BDCKOPABCDN")]
    pub bdckopabcdn: Option<String>,

    #[serde(rename = "EFGPKNMCMPP")]
    pub efgpknmcmpp: i64,

    #[serde(rename = "LIKEOLHBHEF")]
    pub likeolhbhef: i64,

    #[serde(rename = "HKGJECNHBHB")]
    pub hkgjecnhbhb: i64,

    #[serde(rename = "IIDNDIPIMOA")]
    pub iidndipimoa: Option<i64>,

    #[serde(rename = "GPPCGFAAKBN")]
    pub gppcgfaakbn: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub enum Omdhkajhhlk {
    #[serde(rename = "BRICK_ROTATE_45")]
    BrickRotate45,

    #[serde(rename = "BRICK_ROTATE_90")]
    BrickRotate90,
}
