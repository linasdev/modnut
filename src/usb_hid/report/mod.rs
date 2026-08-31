use crate::usb_hid::report::field::UsbHidReportField;

pub mod builder;
pub mod error;
pub mod feature;
pub mod field;

#[derive(Debug)]
pub struct UsbHidReportDescriptor {
    input_reports: Vec<UsbHidReport>,
    output_reports: Vec<UsbHidReport>,
    feature_reports: Vec<UsbHidReport>,
}

#[derive(Debug)]
pub struct UsbHidReport {
    report_id: Option<u8>,
    size_in_bits: usize,
    fields: Vec<UsbHidReportField>,
}

impl UsbHidReportDescriptor {
    pub fn input_reports(&self) -> impl Iterator<Item = &UsbHidReport> {
        self.input_reports.iter()
    }

    pub fn output_reports(&self) -> impl Iterator<Item = &UsbHidReport> {
        self.output_reports.iter()
    }

    pub fn feature_reports(&self) -> impl Iterator<Item = &UsbHidReport> {
        self.feature_reports.iter()
    }
}

impl UsbHidReport {
    pub fn new(report_id: Option<u8>, fields: Vec<UsbHidReportField>) -> Self {
        let size_in_bits = 8 + fields
            .iter()
            .map(UsbHidReportField::size_in_bits)
            .sum::<usize>();

        Self {
            report_id,
            size_in_bits,
            fields,
        }
    }

    pub fn report_id(&self) -> Option<u8> {
        self.report_id
    }

    pub fn size_in_bits(&self) -> usize {
        self.size_in_bits
    }

    pub fn size_in_bytes(&self) -> usize {
        self.size_in_bits().div_ceil(8)
    }

    pub fn fields(&self) -> impl Iterator<Item = &UsbHidReportField> {
        self.fields.iter()
    }
}
