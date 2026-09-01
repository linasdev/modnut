use crate::device::UpsDevice;
use crate::driver::UpsDriver;
use crate::error::ModNutError;
use crate::settings::ModNutUpsDriverSettings;
use crate::usb_hid::device::UpsDeviceUsbHid;
use crate::usb_hid::settings::ModNutUpsDriverUsbHidSettings;
use hidapi::HidApi;
use std::collections::BTreeSet;

pub struct UpsDriverUsbHid {
    hid_api: HidApi,
    settings: Vec<ModNutUpsDriverUsbHidSettings>,
}

impl UpsDriverUsbHid {
    pub fn new(settings: &ModNutUpsDriverSettings) -> Result<Self, ModNutError> {
        let hid_api = HidApi::new()?;

        Ok(Self {
            hid_api,
            settings: settings.usb_hid.clone(),
        })
    }
}

impl UpsDriver for UpsDriverUsbHid {
    fn scan_for_configured_ups_devices(&mut self) -> Result<Vec<Box<dyn UpsDevice>>, ModNutError> {
        let ups_device_paths = self
            .hid_api
            .device_list()
            .filter_map(|device_info| {
                let device_settings = self.settings.iter().find(|settings| {
                    settings.vendor_id == device_info.vendor_id()
                        && settings.product_id == device_info.product_id()
                });

                if let Some(device_settings) = device_settings {
                    Some((device_settings.name.clone(), device_info.path()))
                } else {
                    None
                }
            })
            .collect::<BTreeSet<_>>();

        let ups_devices = ups_device_paths
            .into_iter()
            .map(|(ups_device_name, ups_device_path)| {
                match self.hid_api.open_path(ups_device_path) {
                    Ok(hid_device) => match UpsDeviceUsbHid::new(ups_device_name, hid_device) {
                        Ok(ups_device) => Ok(Box::new(ups_device) as Box<dyn UpsDevice>),
                        Err(error) => Err(error),
                    },
                    Err(error) => Err(error.into()),
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(ups_devices)
    }
}
