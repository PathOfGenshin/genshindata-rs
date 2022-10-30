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

pub type ProductIdConfigData = Vec<ProductIdConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ProductIdConfigDatum {
    #[serde(rename = "productId")]
    pub product_id: String,

    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "isInternal")]
    pub is_internal: Option<bool>,

    #[serde(rename = "entitlementId")]
    pub entitlement_id: String,

    #[serde(rename = "JNJONAJEAOF")]
    pub jnjonajeaof: String,

    #[serde(rename = "HMIJHOPHDOH")]
    pub hmijhophdoh: Vec<Hmijhophdoh>,
}

#[derive(Serialize, Deserialize)]
pub enum Hmijhophdoh {
    #[serde(rename = "CLOUD_ANDROID")]
    CloudAndroid,

    #[serde(rename = "CLOUD_IOS")]
    CloudIos,

    #[serde(rename = "CLOUD_MAC")]
    CloudMac,

    #[serde(rename = "CLOUD_PC")]
    CloudPc,
}
