use crate::driver::UpsDriver;
use crate::error::ModNutError;
use crate::usb_hid::driver::UpsDriverUsbHid;

pub mod device;
pub mod driver;
pub mod error;
pub mod usb_hid;

fn main() -> Result<(), ModNutError> {
    env_logger::init();

    UpsDriverUsbHid::new()?.scan_for_configured_ups_devices()?;

    Ok(())
}
