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

pub type PassCatalogDataData = Vec<PassCatalogDataDatum>;

#[derive(Serialize, Deserialize)]
pub struct PassCatalogDataDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "FFMABNDFCAH")]
    pub ffmabndfcah: i64,

    #[serde(rename = "FDHILJODJCM")]
    pub fdhiljodjcm: i64,

    #[serde(rename = "GJIEHFHGJPE")]
    pub gjiehfhgjpe: String,

    #[serde(rename = "JDPGGCCINLP")]
    pub jdpggccinlp: Jdpggccinlp,

    #[serde(rename = "OFDHEBELPOD")]
    pub ofdhebelpod: String,

    #[serde(rename = "BMMFLGPKCJD")]
    pub bmmflgpkcjd: Vec<Bmmflgpkcjd>,

    #[serde(rename = "JNHEPMIIOLL")]
    pub jnhepmiioll: Option<bool>,

    #[serde(rename = "ECMPKMEEOOP")]
    pub ecmpkmeeoop: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Bmmflgpkcjd {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "JMGDMFDABJC")]
    pub jmgdmfdabjc: Jmgdmfdabjc,

    #[serde(rename = "ECMPKMEEOOP")]
    pub ecmpkmeeoop: Option<i64>,

    #[serde(rename = "weight")]
    pub weight: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Jmgdmfdabjc {
    #[serde(rename = "DeshretPoint_jinruzhiyin01")]
    DeshretPointJinruzhiyin01,

    #[serde(rename = "DeshretPoint_jinruzhiyin02")]
    DeshretPointJinruzhiyin02,

    #[serde(rename = "DeshretPoint_jinruzhiyin03")]
    DeshretPointJinruzhiyin03,

    #[serde(rename = "DeshretPoint_jisichuansongzhiyin")]
    DeshretPointJisichuansongzhiyin,

    #[serde(rename = "DeshretPoint_jisitoliantongzhiyin01")]
    DeshretPointJisitoliantongzhiyin01,

    #[serde(rename = "DeshretPoint_jisitoliantongzhiyin02")]
    DeshretPointJisitoliantongzhiyin02,

    #[serde(rename = "DeshretPoint_kuangdongzhiyin01")]
    DeshretPointKuangdongzhiyin01,

    #[serde(rename = "DeshretPoint_kuangdongzhiyin02")]
    DeshretPointKuangdongzhiyin02,

    #[serde(rename = "DeshretPoint_kuangdongzhiyin03")]
    DeshretPointKuangdongzhiyin03,

    #[serde(rename = "DeshretPoint_kuangdongzhiyin04")]
    DeshretPointKuangdongzhiyin04,

    #[serde(rename = "DeshretPoint_kuangdongzhiyin05")]
    DeshretPointKuangdongzhiyin05,

    #[serde(rename = "DeshretPoint_kuangdongzhiyin06")]
    DeshretPointKuangdongzhiyin06,

    #[serde(rename = "DeshretPoint_quanzhangzhiyin01")]
    DeshretPointQuanzhangzhiyin01,

    #[serde(rename = "DeshretPoint_quanzhangzhiyin02")]
    DeshretPointQuanzhangzhiyin02,

    #[serde(rename = "DeshretPoint_quanzhangzhiyin04")]
    DeshretPointQuanzhangzhiyin04,

    #[serde(rename = "DeshretPoint_quanzhangzhiyin06")]
    DeshretPointQuanzhangzhiyin06,

    #[serde(rename = "DeshretPoint_quanzhangzhiyin07")]
    DeshretPointQuanzhangzhiyin07,

    #[serde(rename = "DeshretPoint_siyuzhiyin01")]
    DeshretPointSiyuzhiyin01,

    #[serde(rename = "DeshretPoint_siyuzhiyin02")]
    DeshretPointSiyuzhiyin02,

    #[serde(rename = "DeshretPoint_siyuzhiyin03")]
    DeshretPointSiyuzhiyin03,

    #[serde(rename = "DeshretPoint_toumingzhiyin01")]
    DeshretPointToumingzhiyin01,

    #[serde(rename = "DeshretPoint_toumingzhiyin02")]
    DeshretPointToumingzhiyin02,

    #[serde(rename = "DeshretPoint_yurendamenzhiyin")]
    DeshretPointYurendamenzhiyin,

    #[serde(rename = "DeshretPoint_yurendownzhiyin01")]
    DeshretPointYurendownzhiyin01,

    #[serde(rename = "DeshretPoint_yurenupzhiyin01")]
    DeshretPointYurenupzhiyin01,

    #[serde(rename = "DeshretPoint_yurenzhiyin")]
    DeshretPointYurenzhiyin,

    #[serde(rename = "DeshretPoint_yurenzhiyin01")]
    DeshretPointYurenzhiyin01,

    #[serde(rename = "")]
    Empty,
}

#[derive(Serialize, Deserialize)]
pub enum Jdpggccinlp {
    #[serde(rename = "BIGWORLD")]
    Bigworld,

    #[serde(rename = "POLYGON")]
    Polygon,

    #[serde(rename = "SUBAREA")]
    Subarea,
}
