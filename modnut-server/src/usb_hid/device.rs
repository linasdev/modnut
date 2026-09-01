use crate::device::UpsDevice;
use crate::error::ModNutError;
use crate::usb_hid::report::builder::UsbHidReportDescriptorBuilder;
use hid_types::id::KnownUsagePage;
use hid_types::id::usage::power;
use hid_types::item::usage::ExtendedUsage;
use hidapi::{HidDevice, MAX_REPORT_DESCRIPTOR_SIZE};
use log::{info, warn};

pub struct UpsDeviceUsbHid {
    name: String,
    manufacturer_name: Option<String>,
    product_name: Option<String>,
    serial_number: Option<String>,
    hid_device: HidDevice,
}

impl UpsDeviceUsbHid {
    pub fn new(name: String, hid_device: HidDevice) -> Result<Self, ModNutError> {
        let device_info = hid_device.get_device_info()?;
        let manufacturer_name = device_info.manufacturer_string().map(String::from);
        let product_name = device_info.product_string().map(String::from);
        let serial_number = device_info.serial_number().map(String::from);

        info!(
            "Found device on {:?}, vendor_id = {}, product_id = {}, manufacturer_name: {manufacturer_name:?}, product_name: {product_name:?}, serial_number: {serial_number:?}",
            device_info.path(),
            device_info.vendor_id(),
            device_info.product_id(),
        );

        let mut report_descriptor_buffer = [0u8; MAX_REPORT_DESCRIPTOR_SIZE];
        let report_descriptor_size =
            hid_device.get_report_descriptor(&mut report_descriptor_buffer)?;

        let report_descriptor_items =
            hid_decode::decode_items(&report_descriptor_buffer[0..report_descriptor_size])?;

        let report_descriptor =
            UsbHidReportDescriptorBuilder::new().build(report_descriptor_items)?;

        let ups_usage_exists = report_descriptor.root_collections().any(|root_collection| {
            root_collection.usage()
                == ExtendedUsage::new(KnownUsagePage::Power.into(), power::KnownUsage::Ups.into())
        });

        if !ups_usage_exists {
            warn!(
                "No root collection with usage Power.Ups found in device's HID report descriptor"
            );
        }

        Ok(Self {
            name,
            manufacturer_name,
            product_name,
            serial_number,
            hid_device,
        })
    }
}

impl UpsDevice for UpsDeviceUsbHid {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn manufacturer_name(&self) -> Option<String> {
        self.manufacturer_name.clone()
    }

    fn product_name(&self) -> Option<String> {
        self.product_name.clone()
    }

    fn serial_number(&self) -> Option<String> {
        self.serial_number.clone()
    }
}
