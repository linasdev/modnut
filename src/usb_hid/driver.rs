use crate::device::UpsDevice;
use crate::driver::UpsDriver;
use crate::error::ModNutError;
use crate::usb_hid::device::UpsDeviceUsbHid;
use hid_types::id::UsagePage;
use hid_types::id::usage::power;
use hid_types::item::usage::{KnownUsage, Usage};
use hidapi::HidApi;
use std::collections::BTreeSet;

pub struct UpsDriverUsbHid {
    hid_api: HidApi,
}

impl UpsDriverUsbHid {
    pub fn new() -> Result<Self, ModNutError> {
        let hid_api = HidApi::new()?;

        Ok(Self { hid_api })
    }
}

impl UpsDriver for UpsDriverUsbHid {
    fn scan_for_configured_ups_devices(&mut self) -> Result<Vec<Box<dyn UpsDevice>>, ModNutError> {
        let ups_device_paths = self
            .hid_api
            .device_list()
            .filter_map(|device_info| {
                if Usage::Known(KnownUsage::Power(power::KnownUsage::Ups))
                    == Usage::new(
                        UsagePage::from_integer(device_info.usage_page()),
                        device_info.usage(),
                    )
                {
                    Some(device_info.path())
                } else {
                    None
                }
            })
            .collect::<BTreeSet<_>>();

        let ups_devices = ups_device_paths
            .into_iter()
            .map(|ups_device_path| self.hid_api.open_path(ups_device_path))
            .map(|hid_device_result| match hid_device_result {
                Ok(hid_device) => match UpsDeviceUsbHid::new(hid_device) {
                    Ok(ups_device) => Ok(Box::new(ups_device) as Box<dyn UpsDevice>),
                    Err(error) => Err(error),
                },
                Err(error) => Err(error.into()),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(ups_devices)
    }
}
