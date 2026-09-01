pub trait UpsDevice {
    fn name(&self) -> String;
    fn manufacturer_name(&self) -> Option<String>;
    fn product_name(&self) -> Option<String>;
    fn serial_number(&self) -> Option<String>;
}
