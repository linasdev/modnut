use crate::usb_hid::report::error::UsbHidReportError;

#[derive(Debug)]
pub enum ModNutError {
    HidApi(hidapi::HidError),
    HidDecodeLengthError(hid_decode::LengthError),
    UsbHidReport(UsbHidReportError),
}

impl From<hidapi::HidError> for ModNutError {
    fn from(error: hidapi::HidError) -> Self {
        ModNutError::HidApi(error)
    }
}

impl From<hid_decode::LengthError> for ModNutError {
    fn from(error: hid_decode::LengthError) -> Self {
        ModNutError::HidDecodeLengthError(error)
    }
}

impl From<UsbHidReportError> for ModNutError {
    fn from(error: UsbHidReportError) -> Self {
        ModNutError::UsbHidReport(error)
    }
}
