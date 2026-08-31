#[derive(Debug)]
pub enum UsbHidReportError {
    PopBeforePush,
    ReportIdMissing,
    ReportSizeMissing,
    ReportCountMissing,
    UsagePageMissing,
    UsageMissing,
    InvalidUsageRange,
    InvalidDesignatorIndexRange,
    InvalidStringIndexRange,
    InvalidReportId,
    InvalidBufferSize,
    InvalidLogicalValueRange,
    InvalidPhysicalValueRange,
    LogicalValueOutOfRange,
}
