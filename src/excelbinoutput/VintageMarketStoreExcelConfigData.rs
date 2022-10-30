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

pub type VintageMarketStoreExcelConfigData = Vec<VintageMarketStoreExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct VintageMarketStoreExcelConfigDatum {
    #[serde(rename = "MEPDABCIKFD")]
    pub mepdabcikfd: i64,

    #[serde(rename = "MAHDCLCKDJI")]
    pub mahdclckdji: i64,

    #[serde(rename = "NPEKDLNHCPG")]
    pub npekdlnhcpg: i64,

    #[serde(rename = "IPJCFCNJKKI")]
    pub ipjcfcnjkki: Vec<i64>,

    #[serde(rename = "AJLAEJBALCK")]
    pub ajlaejbalck: Vec<Ajlaejbalck>,

    #[serde(rename = "FNPECGOHLDP")]
    pub fnpecgohldp: i64,

    #[serde(rename = "NMNJDBJDCIB")]
    pub nmnjdbjdcib: Vec<i64>,

    #[serde(rename = "npcId")]
    pub npc_id: i64,

    #[serde(rename = "PBMGLKBOKEB")]
    pub pbmglkbokeb: i64,

    #[serde(rename = "DNDHGOIPAEE")]
    pub dndhgoipaee: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Ajlaejbalck {
    #[serde(rename = "defaultValue")]
    pub default_value: i64,

    #[serde(rename = "LFOLKMHENDI")]
    pub lfolkmhendi: i64,
}
