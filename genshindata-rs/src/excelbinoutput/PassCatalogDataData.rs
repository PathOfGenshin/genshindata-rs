/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type PassCatalogDataData = Vec<PassCatalogDataDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct PassCatalogDataDatum {
    #[serde(rename = "id")]
    pub id: i64,
    pub ifpjahdefad: i64,
    pub mmbakcjjjaf: i64,
    pub fehkboljioo: String,
    pub eoeiolgbmgm: Eoeiolgbmgm,
    pub fmcjoihifkl: String,
    pub ificadbiglj: Vec<Ificadbiglj>,
    pub heiimkfpeef: Option<bool>,
    pub pebpmgfhgno: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Eoeiolgbmgm {
    #[serde(rename = "BIGWORLD")]
    Bigworld,
    #[serde(rename = "POLYGON")]
    Polygon,
    #[serde(rename = "SUBAREA")]
    Subarea,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ificadbiglj {
    pub id: Option<i64>,
    #[serde(rename = "AHEAGNHLFNL")]
    pub aheagnhlfnl: Aheagnhlfnl,
    #[serde(rename = "PEBPMGFHGNO")]
    pub pebpmgfhgno: Option<i64>,
    pub weight: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Aheagnhlfnl {
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
