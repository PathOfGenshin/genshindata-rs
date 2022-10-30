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

pub type MusicGameDrumConfigData = Vec<MusicGameDrumConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MusicGameDrumConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "KANNCGBKMNO")]
    pub kanncgbkmno: Vec<f64>,

    #[serde(rename = "DIPEOEIAEKF")]
    pub dipeoeiaekf: Vec<f64>,

    #[serde(rename = "IBHDMGCBCJO")]
    pub ibhdmgcbcjo: Vec<f64>,

    #[serde(rename = "DENIPPFACIB")]
    pub denippfacib: Vec<f64>,

    #[serde(rename = "AMDMHPPOOHJ")]
    pub amdmhppoohj: Vec<f64>,

    #[serde(rename = "OGFCHPBDCCL")]
    pub ogfchpbdccl: i64,

    #[serde(rename = "EBJNDMJBBDA")]
    pub ebjndmjbbda: i64,

    #[serde(rename = "LNHAGFELLIE")]
    pub lnhagfellie: i64,

    #[serde(rename = "PKEKAGMNLKJ")]
    pub pkekagmnlkj: i64,

    #[serde(rename = "HMPBIFGLLMG")]
    pub hmpbifgllmg: i64,

    #[serde(rename = "MGDKKBANPHG")]
    pub mgdkkbanphg: i64,

    #[serde(rename = "CKKMLBOABGN")]
    pub ckkmlboabgn: i64,

    #[serde(rename = "KOCEHAGFHJK")]
    pub kocehagfhjk: Vec<f64>,

    #[serde(rename = "EIGCNIIMOCC")]
    pub eigcniimocc: f64,

    #[serde(rename = "OJHJEAFKIJA")]
    pub ojhjeafkija: Vec<f64>,

    #[serde(rename = "AAFGMBAEOCO")]
    pub aafgmbaeoco: i64,

    #[serde(rename = "HOFBADHOFAC")]
    pub hofbadhofac: f64,

    #[serde(rename = "FFHGKABKFKK")]
    pub ffhgkabkfkk: f64,

    #[serde(rename = "NLEPAJJFFKN")]
    pub nlepajjffkn: i64,

    #[serde(rename = "KNCOGGKIJDF")]
    pub kncoggkijdf: Vec<f64>,

    #[serde(rename = "OEBHNEHHDIF")]
    pub oebhnehhdif: Vec<f64>,

    #[serde(rename = "AKHHGBJMHAN")]
    pub akhhgbjmhan: Vec<f64>,

    #[serde(rename = "KALHFLEIAID")]
    pub kalhfleiaid: Vec<f64>,

    #[serde(rename = "EOAGJAKOILC")]
    pub eoagjakoilc: i64,

    #[serde(rename = "JHEFOGAPBGO")]
    pub jhefogapbgo: Vec<i64>,

    #[serde(rename = "HHGPNKCIFDL")]
    pub hhgpnkcifdl: Vec<i64>,

    #[serde(rename = "DNLFBONOHJN")]
    pub dnlfbonohjn: Vec<i64>,

    #[serde(rename = "IKNOKHPDDIN")]
    pub iknokhpddin: Vec<f64>,

    #[serde(rename = "KACPOGAOFPD")]
    pub kacpogaofpd: Vec<f64>,

    #[serde(rename = "PAHAAENJIPK")]
    pub pahaaenjipk: Vec<i64>,
}
