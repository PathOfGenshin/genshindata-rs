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

pub type AvatarCostumeExcelConfigData = Vec<AvatarCostumeExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct AvatarCostumeExcelConfigDatum {
    #[serde(rename = "OGKFGGNLLDG")]
    pub ogkfggnlldg: i64,

    #[serde(rename = "KCLGNCODKCB")]
    pub kclgncodkcb: Option<i64>,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "itemId")]
    pub item_id: Option<i64>,

    #[serde(rename = "AKOANLMAFDD")]
    pub akoanlmafdd: i64,

    #[serde(rename = "jsonName")]
    pub json_name: String,

    #[serde(rename = "POKBOHJOIAO")]
    pub pokbohjoiao: Option<i64>,

    #[serde(rename = "KBOJMHOEIGP")]
    pub kbojmhoeigp: Option<i64>,

    #[serde(rename = "JHLHOFMIACH")]
    pub jhlhofmiach: Option<i64>,

    #[serde(rename = "MOBDPMDPOLK")]
    pub mobdpmdpolk: Option<i64>,

    #[serde(rename = "CPCBAHOICOI")]
    pub cpcbahoicoi: Option<i64>,

    #[serde(rename = "EAPJFIFAKJD")]
    pub eapjfifakjd: Option<i64>,

    #[serde(rename = "HLAPLALPJMA")]
    pub hlaplalpjma: Option<i64>,

    #[serde(rename = "PKKOEBGNGDI")]
    pub pkkoebgngdi: Option<i64>,

    #[serde(rename = "KCGENIAAGAN")]
    pub kcgeniaagan: Option<i64>,

    #[serde(rename = "IFIODPDADEI")]
    pub ifiodpdadei: String,

    #[serde(rename = "sideIconName")]
    pub side_icon_name: String,

    #[serde(rename = "CAMHHHOEOKL")]
    pub camhhhoeokl: Option<i64>,

    #[serde(rename = "FNBIOFBAAOE")]
    pub fnbiofbaaoe: Option<i64>,

    #[serde(rename = "MKBIFLBNLON")]
    pub mkbiflbnlon: Option<bool>,

    #[serde(rename = "hide")]
    pub hide: Option<bool>,

    #[serde(rename = "gachaCardNameHashSuffix")]
    pub gacha_card_name_hash_suffix: Option<i64>,

    #[serde(rename = "coopPicNameHashSuffix")]
    pub coop_pic_name_hash_suffix: Option<i64>,

    #[serde(rename = "IPDGIHCGHNI")]
    pub ipdgihcghni: Option<i64>,

    #[serde(rename = "DHPHIEMEEBB")]
    pub dhphiemeebb: Option<i64>,

    #[serde(rename = "HIPJFMBAEMO")]
    pub hipjfmbaemo: Option<i64>,

    #[serde(rename = "FFJDLEKIFBF")]
    pub ffjdlekifbf: Option<i64>,

    #[serde(rename = "isDefault")]
    pub is_default: Option<bool>,

    #[serde(rename = "ENBLJGDBMMK")]
    pub enbljgdbmmk: Option<bool>,
}
