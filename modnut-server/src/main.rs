use crate::driver::UpsDriver;
use crate::error::ModNutError;
use crate::settings::ModNutSettings;
use crate::usb_hid::driver::UpsDriverUsbHid;
use config::Config;

pub mod device;
pub mod driver;
pub mod error;
pub mod settings;
pub mod usb_hid;

fn main() -> Result<(), ModNutError> {
    env_logger::init();

    let settings: ModNutSettings = Config::builder()
        .add_source(config::File::new("modnut.toml", config::FileFormat::Toml).required(false))
        .add_source(config::Environment::with_prefix("MODNUT"))
        .build()
        .expect("Failed to build config")
        .try_deserialize()
        .expect("Failed to deserialize config");

    UpsDriverUsbHid::new(&settings.driver)?.scan_for_configured_ups_devices()?;

    Ok(())
}
