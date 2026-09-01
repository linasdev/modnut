use crate::usb_hid::settings::ModNutUpsDriverUsbHidSettings;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ModNutSettings {
    pub driver: ModNutUpsDriverSettings,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ModNutUpsDriverSettings {
    pub usb_hid: Vec<ModNutUpsDriverUsbHidSettings>,
}
