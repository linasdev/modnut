use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ModNutUpsDriverUsbHidSettings {
    pub name: String,
    pub vendor_id: u16,
    pub product_id: u16,
}
