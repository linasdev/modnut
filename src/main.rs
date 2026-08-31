use crate::error::ModNutError;

pub mod device;
pub mod driver;
pub mod error;
pub mod settings;
pub mod usb_hid;

fn main() -> Result<(), ModNutError> {
    env_logger::init();

    Ok(())
}
