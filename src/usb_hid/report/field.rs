use crate::usb_hid::report::error::UsbHidReportError;
use hid_types::id::{Unit, UnitExponent};
use hid_types::item::usage::ExtendedUsage;
use std::ops::{Div, Range, Rem, Sub};

#[derive(Debug)]
pub enum UsbHidReportField {
    Variable(UsbHidReportVariableField),
    Array(UsbHidReportArrayField),
    Padding(UsbHidReportPaddingField),
}

#[derive(Debug)]
pub struct UsbHidReportVariableField {
    pub is_constant: bool,
    pub report_id: Option<u8>,
    pub usage: ExtendedUsage,
    pub designator_index: Option<u32>,
    pub string_index: Option<u32>,
    pub bits: Range<u32>,
    pub logical_minimum: Option<i32>,
    pub logical_maximum: Option<i32>,
    pub physical_minimum: Option<i32>,
    pub physical_maximum: Option<i32>,
    pub unit_exponent: Option<UnitExponent>,
    pub unit: Option<Unit>,
}

#[derive(Debug)]
pub struct UsbHidReportArrayField {
    pub is_constant: bool,
    pub report_id: Option<u8>,
    pub usages: Vec<ExtendedUsage>,
    pub designator_indices: Vec<u32>,
    pub string_indices: Vec<u32>,
    pub bits: Range<u32>,
    pub logical_minimum: Option<i32>,
    pub logical_maximum: Option<i32>,
}

#[derive(Debug)]
pub struct UsbHidReportPaddingField {
    pub bits: Range<u32>,
}

impl UsbHidReportField {
    pub fn size_in_bits(&self) -> usize {
        match self {
            UsbHidReportField::Variable(variable_field) => variable_field.bits.len(),
            UsbHidReportField::Array(array_field) => array_field.bits.len(),
            UsbHidReportField::Padding(padding_field) => padding_field.bits.len(),
        }
    }

    pub fn bits(&self) -> &Range<u32> {
        match self {
            UsbHidReportField::Variable(variable_field) => &variable_field.bits,
            UsbHidReportField::Array(array_field) => &array_field.bits,
            UsbHidReportField::Padding(padding_field) => &padding_field.bits,
        }
    }
}

impl UsbHidReportVariableField {
    pub fn extract_value(&self, buffer: &[u8]) -> Result<Vec<u8>, UsbHidReportError> {
        extract_value(self.report_id, &self.bits, buffer)
    }

    pub fn extract_logical_value(&self, buffer: &[u8]) -> Result<Option<i64>, UsbHidReportError> {
        extract_logical_value(
            self.report_id,
            &self.bits,
            self.logical_minimum,
            self.logical_maximum,
            buffer,
            true,
        )
    }

    pub fn extract_physical_value(&self, buffer: &[u8]) -> Result<Option<f64>, UsbHidReportError> {
        let Some(logical_value) = self.extract_logical_value(buffer)? else {
            return Ok(None);
        };

        let Some(logical_minimum) = self.logical_minimum.map(i64::from) else {
            return Ok(None);
        };

        let Some(logical_maximum) = self.logical_maximum.map(i64::from) else {
            return Ok(None);
        };

        let (physical_minimum, physical_maximum) = if let Some(physical_minimum) =
            self.physical_minimum
            && let Some(physical_maximum) = self.physical_maximum
        {
            if physical_minimum != 0 || physical_maximum != 0 {
                (physical_minimum as i64, physical_maximum as i64)
            } else {
                (logical_minimum, logical_maximum)
            }
        } else {
            (logical_minimum, logical_maximum)
        };

        if logical_maximum <= logical_minimum {
            return Err(UsbHidReportError::InvalidLogicalValueRange);
        }

        if physical_maximum <= physical_minimum {
            return Err(UsbHidReportError::InvalidPhysicalValueRange);
        }

        let unscaled_physical_value = physical_minimum as f64
            + (logical_value - logical_minimum) as f64 / (logical_maximum - logical_minimum) as f64
                * (physical_maximum - physical_minimum) as f64;
        let physical_value = unscaled_physical_value
            * 10f64.powi(
                self.unit_exponent
                    .map(UnitExponent::remap_as_i4)
                    .unwrap_or(0),
            );

        Ok(Some(physical_value))
    }
}

impl UsbHidReportArrayField {
    pub fn extract_value(&self, buffer: &[u8]) -> Result<Vec<u8>, UsbHidReportError> {
        extract_value(self.report_id, &self.bits, buffer)
    }

    pub fn extract_logical_value(&self, buffer: &[u8]) -> Result<Option<i64>, UsbHidReportError> {
        extract_logical_value(
            self.report_id,
            &self.bits,
            self.logical_minimum,
            self.logical_maximum,
            buffer,
            false,
        )
    }
}

fn extract_value(
    report_id: Option<u8>,
    bits: &Range<u32>,
    buffer: &[u8],
) -> Result<Vec<u8>, UsbHidReportError> {
    if let Some(report_id) = report_id
        && report_id != buffer[0]
    {
        return Err(UsbHidReportError::InvalidReportId);
    }

    if buffer.len() < bits.end.div_ceil(8) as usize {
        return Err(UsbHidReportError::InvalidBufferSize);
    }

    let mut value = vec![0u8; bits.len().div_ceil(8)];

    for (value_bit_index, buffer_bit_index) in bits.clone().enumerate() {
        let value_byte_index = value_bit_index.div(8);
        let buffer_byte_index = buffer_bit_index.div(8) as usize;

        let buffer_bit = (buffer[buffer_byte_index] >> buffer_bit_index.rem(8)) & 1;
        let value_mask = buffer_bit << value_bit_index.rem(8);

        value[value_byte_index] |= value_mask;
    }

    Ok(value)
}

fn extract_logical_value(
    report_id: Option<u8>,
    bits: &Range<u32>,
    logical_minimum: Option<i32>,
    logical_maximum: Option<i32>,
    buffer: &[u8],
    validate_value_range: bool,
) -> Result<Option<i64>, UsbHidReportError> {
    let value_width = bits.len();
    if value_width > 32 {
        return Ok(None);
    }

    if let Some(logical_minimum) = logical_minimum
        && let Some(logical_maximum) = logical_maximum
        && logical_maximum <= logical_minimum
    {
        return Err(UsbHidReportError::InvalidLogicalValueRange);
    }

    let value = extract_value(report_id, bits, buffer)?;

    let Some(logical_minimum) = logical_minimum.map(i64::from) else {
        return Ok(None);
    };

    let logical_maximum = if let Some(logical_maximum) = logical_maximum {
        if logical_minimum.is_negative() {
            logical_maximum as i64
        } else {
            (logical_maximum as u32) as i64
        }
    } else {
        return Ok(None);
    };

    let value_negative = value
        .last()
        .map(|most_significant_byte| {
            let most_significant_bit_index = value_width.sub(1).rem(8);
            most_significant_byte & (1 << most_significant_bit_index) != 0
        })
        .unwrap_or(false);

    let mut value_buffer = [0u8; size_of::<i64>()];
    value_buffer[..value.len()].copy_from_slice(&value);

    let logical_value = {
        let logical_value = u64::from_le_bytes(value_buffer);
        if logical_minimum.is_negative() && value_negative {
            let sign_extension_shift = 64 - value_width;
            (logical_value << sign_extension_shift) as i64 >> sign_extension_shift
        } else {
            logical_value as i64
        }
    };

    if !(logical_minimum..=logical_maximum).contains(&logical_value) {
        if validate_value_range {
            Err(UsbHidReportError::LogicalValueOutOfRange)
        } else {
            Ok(None)
        }
    } else {
        Ok(Some(logical_value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;
    use hid_types::id::usage::power;
    use hid_types::id::{KnownUnit, KnownUsagePage};

    #[test]
    fn should_extract_value_1() {
        let report_id = Some(0x55);
        let bits = 11..15;
        let buffer = vec![0x55; 15];

        let result = extract_value(report_id, &bits, &buffer);
        assert_that!(result, matches_pattern!(Ok(elements_are![eq(&0x0a),])));
    }

    #[test]
    fn should_extract_value_2() {
        let report_id = Some(0x55);
        let bits = 8..(15 * 8);
        let buffer = vec![0x55; 15];

        let result = extract_value(report_id, &bits, &buffer);
        assert_that!(result, matches_pattern!(Ok(eq(&buffer[1..]))));
    }

    #[test]
    fn should_extract_value_3() {
        let report_id = Some(0x55);
        let bits = 8..8;
        let buffer = vec![0x55; 15];

        let result = extract_value(report_id, &bits, &buffer);
        assert_that!(result, matches_pattern!(Ok(is_empty())));
    }

    #[test]
    fn should_extract_value_4() {
        let report_id = Some(0x55);
        let bits = 8..24;
        let buffer = vec![0x55; 15];

        let result = extract_value(report_id, &bits, &buffer);
        assert_that!(
            result,
            matches_pattern!(Ok(elements_are![eq(&0x55), eq(&0x55),]))
        );
    }

    #[test]
    fn should_not_extract_value_when_report_id_is_invalid() {
        let report_id = Some(0x11);
        let bits = 11..15;
        let buffer = vec![0x55; 15];

        let result = extract_value(report_id, &bits, &buffer);
        assert_that!(
            result,
            matches_pattern!(Err(matches_pattern!(UsbHidReportError::InvalidReportId)))
        );
    }

    #[test]
    fn should_not_extract_value_when_buffer_is_too_small() {
        let report_id = Some(0x55);
        let bits = 16..24;
        let buffer = vec![0x55; 2];

        let result = extract_value(report_id, &bits, &buffer);
        assert_that!(
            result,
            matches_pattern!(Err(matches_pattern!(UsbHidReportError::InvalidBufferSize)))
        );
    }

    #[test]
    fn should_extract_logical_value_1() {
        let report_id = Some(0x55);
        let bits = 11..14;
        let logical_minimum = Some(-128);
        let logical_maximum = Some(127);
        let buffer = vec![0x55; 15];

        let result = extract_logical_value(
            report_id,
            &bits,
            logical_minimum,
            logical_maximum,
            &buffer,
            true,
        );
        assert_that!(
            result,
            matches_pattern!(Ok(matches_pattern!(Some(eq(&2),))))
        );
    }

    #[test]
    fn should_extract_logical_value_2() {
        let report_id = Some(0x55);
        let bits = 11..15;
        let logical_minimum = Some(-128);
        let logical_maximum = Some(127);
        let buffer = vec![0x55; 15];

        let result = extract_logical_value(
            report_id,
            &bits,
            logical_minimum,
            logical_maximum,
            &buffer,
            true,
        );
        assert_that!(
            result,
            matches_pattern!(Ok(matches_pattern!(Some(eq(&-6),))))
        );
    }

    #[test]
    fn should_extract_logical_value_3() {
        let report_id = Some(0xff);
        let bits = 8..40;
        let logical_minimum = Some(i32::MIN);
        let logical_maximum = Some(i32::MAX);
        let buffer = vec![0xff; 15];

        let result = extract_logical_value(
            report_id,
            &bits,
            logical_minimum,
            logical_maximum,
            &buffer,
            true,
        );
        assert_that!(
            result,
            matches_pattern!(Ok(matches_pattern!(Some(eq(&-1),))))
        );
    }

    #[test]
    fn should_extract_logical_value_4() {
        let report_id = Some(0x00);
        let bits = 8..40;
        let logical_minimum = Some(i32::MIN);
        let logical_maximum = Some(i32::MAX);
        let buffer = {
            let mut buffer = vec![0x00; 15];
            buffer[4] = 0b1000_0000;
            buffer
        };

        let result = extract_logical_value(
            report_id,
            &bits,
            logical_minimum,
            logical_maximum,
            &buffer,
            true,
        );
        assert_that!(
            result,
            matches_pattern!(Ok(matches_pattern!(Some(eq(&(i32::MIN as i64)),))))
        );
    }

    #[test]
    fn should_not_extract_logical_value_when_report_id_is_invalid() {
        let report_id = Some(0x11);
        let bits = 11..14;
        let logical_minimum = Some(-128);
        let logical_maximum = Some(127);
        let buffer = vec![0x55; 15];

        let result = extract_logical_value(
            report_id,
            &bits,
            logical_minimum,
            logical_maximum,
            &buffer,
            true,
        );
        assert_that!(
            result,
            matches_pattern!(Err(matches_pattern!(UsbHidReportError::InvalidReportId)))
        );
    }

    #[test]
    fn should_not_extract_logical_value_when_logical_minimum_above_maximum() {
        let report_id = Some(0x55);
        let bits = 11..14;
        let logical_minimum = Some(127);
        let logical_maximum = Some(-128);
        let buffer = vec![0x55; 15];

        let result = extract_logical_value(
            report_id,
            &bits,
            logical_minimum,
            logical_maximum,
            &buffer,
            true,
        );
        assert_that!(
            result,
            matches_pattern!(Err(matches_pattern!(
                UsbHidReportError::InvalidLogicalValueRange
            )))
        );
    }

    #[test]
    fn should_not_extract_logical_value_when_logical_value_is_out_of_range() {
        let report_id = Some(0x55);
        let bits = 9..17;
        let logical_minimum = Some(-1);
        let logical_maximum = Some(i32::MAX);
        let buffer = vec![0x55; 15];

        let result = extract_logical_value(
            report_id,
            &bits,
            logical_minimum,
            logical_maximum,
            &buffer,
            true,
        );
        assert_that!(
            result,
            matches_pattern!(Err(matches_pattern!(
                UsbHidReportError::LogicalValueOutOfRange
            )))
        );
    }

    #[test]
    fn should_not_extract_logical_value_when_logical_value_is_out_of_range_and_validate_value_range_is_false()
     {
        let report_id = Some(0x55);
        let bits = 9..17;
        let logical_minimum = Some(-1);
        let logical_maximum = Some(i32::MAX);
        let buffer = vec![0x55; 15];

        let result = extract_logical_value(
            report_id,
            &bits,
            logical_minimum,
            logical_maximum,
            &buffer,
            false,
        );
        assert_that!(result, matches_pattern!(Ok(matches_pattern!(None))));
    }

    #[test]
    fn should_extract_physical_value_1() {
        let target = UsbHidReportVariableField {
            is_constant: false,
            report_id: Some(127),
            usage: ExtendedUsage::new(KnownUsagePage::Power.into(), power::KnownUsage::Ups.into()),
            designator_index: None,
            string_index: None,
            bits: 8..16,
            logical_minimum: Some(-127),
            logical_maximum: Some(127),
            physical_minimum: Some(-3175),
            physical_maximum: Some(3175),
            unit_exponent: Some(UnitExponent::from(-4)),
            unit: Some(Unit::Known(KnownUnit::Inch)),
        };

        let buffer = vec![127; 2];
        let result = target.extract_physical_value(&buffer);
        assert_that!(
            result,
            matches_pattern!(Ok(matches_pattern!(Some(eq(&0.3175)))))
        );
    }

    #[test]
    fn should_extract_physical_value_2() {
        let target = UsbHidReportVariableField {
            is_constant: false,
            report_id: Some((-127i8) as u8),
            usage: ExtendedUsage::new(KnownUsagePage::Power.into(), power::KnownUsage::Ups.into()),
            designator_index: None,
            string_index: None,
            bits: 8..16,
            logical_minimum: Some(-127),
            logical_maximum: Some(127),
            physical_minimum: Some(-3175),
            physical_maximum: Some(3175),
            unit_exponent: Some(UnitExponent::from(-4)),
            unit: Some(Unit::Known(KnownUnit::Inch)),
        };

        let buffer = vec![(-127i8) as u8; 2];
        let result = target.extract_physical_value(&buffer);
        assert_that!(
            result,
            matches_pattern!(Ok(matches_pattern!(Some(eq(&-0.3175)))))
        );
    }

    #[test]
    fn should_extract_physical_value_3() {
        let target = UsbHidReportVariableField {
            is_constant: false,
            report_id: Some(0),
            usage: ExtendedUsage::new(KnownUsagePage::Power.into(), power::KnownUsage::Ups.into()),
            designator_index: None,
            string_index: None,
            bits: 8..16,
            logical_minimum: Some(-127),
            logical_maximum: Some(127),
            physical_minimum: Some(-3175),
            physical_maximum: Some(3175),
            unit_exponent: Some(UnitExponent::from(-4)),
            unit: Some(Unit::Known(KnownUnit::Inch)),
        };

        let buffer = vec![0; 2];
        let result = target.extract_physical_value(&buffer);
        assert_that!(
            result,
            matches_pattern!(Ok(matches_pattern!(Some(eq(&0.0)))))
        );
    }

    #[test]
    fn should_not_extract_physical_value_when_physical_minimum_above_maximum() {
        let target = UsbHidReportVariableField {
            is_constant: false,
            report_id: Some(0),
            usage: ExtendedUsage::new(KnownUsagePage::Power.into(), power::KnownUsage::Ups.into()),
            designator_index: None,
            string_index: None,
            bits: 8..16,
            logical_minimum: Some(-127),
            logical_maximum: Some(127),
            physical_minimum: Some(3175),
            physical_maximum: Some(-3175),
            unit_exponent: Some(UnitExponent::from(-4)),
            unit: Some(Unit::Known(KnownUnit::Inch)),
        };

        let buffer = vec![0; 2];
        let result = target.extract_physical_value(&buffer);
        assert_that!(
            result,
            matches_pattern!(Err(matches_pattern!(
                UsbHidReportError::InvalidPhysicalValueRange
            )))
        );
    }
}
