use crate::device::UpsDevice;
use crate::error::ModNutError;
use crate::usb_hid::report::builder::UsbHidReportDescriptorBuilder;
use crate::usb_hid::report::field::UsbHidReportField;
use hidapi::{HidDevice, MAX_REPORT_DESCRIPTOR_SIZE};
use log::{info, warn};

pub enum UpsDeviceIdentifierUsbHid {
    Serial(String),
    PathAndInterfaceNumber(String, u32),
}

pub struct UpsDeviceUsbHid {
    hid_device: HidDevice,
}

impl UpsDeviceUsbHid {
    pub fn new(hid_device: HidDevice) -> Result<Self, ModNutError> {
        let device_info = hid_device.get_device_info()?;

        info!(
            "Found UPS on {:?}, vendor_id = {}, product_id = {}, manufacturer_name: {:?}, product_name: {:?}",
            device_info.path(),
            device_info.vendor_id(),
            device_info.product_id(),
            device_info.manufacturer_string(),
            device_info.product_string(),
        );

        let mut report_descriptor_buffer = [0u8; MAX_REPORT_DESCRIPTOR_SIZE];
        let report_descriptor_size =
            hid_device.get_report_descriptor(&mut report_descriptor_buffer)?;

        let report_descriptor_items =
            hid_decode::decode_items(&report_descriptor_buffer[0..report_descriptor_size])?;

        let report_descriptor =
            UsbHidReportDescriptorBuilder::new().build(report_descriptor_items)?;

        for feature_report in report_descriptor.feature_reports() {
            // if let Some(report_id) = feature_report.report_id()
            //     && report_id != 1
            // {
            //     continue;
            // }

            let mut feature_report_buffer = vec![0u8; feature_report.size_in_bytes()];
            feature_report_buffer[0] = feature_report.report_id().unwrap_or(0);
            let feature_report_size = hid_device.get_feature_report(&mut feature_report_buffer)?;

            for field in feature_report.fields() {
                match field {
                    UsbHidReportField::Variable(variable_field) => {
                        let field_value = variable_field
                            .extract_physical_value(&feature_report_buffer[..feature_report_size])
                            .ok()
                            .flatten();
                        println!(
                            "VAR {:?} {:?} {:?}",
                            variable_field.usage, variable_field.bits, field_value
                        );
                    }
                    UsbHidReportField::Array(array_field) => {
                        let field_value = array_field
                            .extract_logical_value(&feature_report_buffer[..feature_report_size])
                            .ok()
                            .flatten();
                        println!(
                            "ARR {:?} {:?} {:?}",
                            array_field.usages, array_field.bits, field_value
                        );
                    }
                    UsbHidReportField::Padding(padding_field) => {
                        println!("PAD {:?}", padding_field.bits);
                    }
                }
            }

            warn!("");
            warn!("");
            warn!("");
        }

        Ok(Self { hid_device })
    }
}

impl UpsDevice for UpsDeviceUsbHid {}
