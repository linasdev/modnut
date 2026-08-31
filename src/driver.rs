use crate::device::UpsDevice;
use crate::error::ModNutError;

pub trait UpsDriver {
    fn scan_for_configured_ups_devices(&mut self) -> Result<Vec<Box<dyn UpsDevice>>, ModNutError>;
}
