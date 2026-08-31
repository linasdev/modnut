use crate::usb_hid::device::UpsDeviceIdentifierUsbHid;

pub enum UpsDeviceIdentifier {
    UsbHid(UpsDeviceIdentifierUsbHid),
}

pub trait UpsDevice {
    // fn identifier(&self) -> UpsIdentifier;
    // fn manufacturer_name(&self) -> Option<String>;
    // fn product_name(&self) -> Option<String>;
}
