use crate::string_escape::escape_string_value;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct ModNutVariableName(pub String);

impl Display for ModNutVariableName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct ModNutVariableDescription(pub String);

impl Display for ModNutVariableDescription {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", escape_string_value(self.0.as_str()))
    }
}

#[derive(Debug, Clone)]
pub enum ModNutVariableType {
    // RW
    ReadWrite,
    // ENUM
    Enumeration,
    // STRING:n
    String(u64),
    // RANGE
    Range,
    // NUMBER
    Number,
}

impl Display for ModNutVariableType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ModNutVariableType::ReadWrite => write!(f, "RW"),
            ModNutVariableType::Enumeration => write!(f, "ENUM"),
            ModNutVariableType::String(max_length) => write!(f, "STRING:{max_length}"),
            ModNutVariableType::Range => write!(f, "RANGE"),
            ModNutVariableType::Number => write!(f, "NUMBER"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModNutVariableTypeEnumVariant(pub String);

impl Display for ModNutVariableTypeEnumVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", escape_string_value(self.0.as_str()))
    }
}

#[derive(Debug, Clone)]
pub enum ModNutVariableTypeRangeBound {
    Integer(u64),
    Float(f64),
}

impl Display for ModNutVariableTypeRangeBound {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ModNutVariableTypeRangeBound::Integer(value) => write!(f, "{value}"),
            ModNutVariableTypeRangeBound::Float(value) => write!(f, "{value}"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ModNutVariableValue {
    String(String),
    Integer(u64),
    Float(f64),
}

impl Display for ModNutVariableValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ModNutVariableValue::String(value) => write!(f, "{}", escape_string_value(value)),
            ModNutVariableValue::Integer(value) => write!(f, "{value}"),
            ModNutVariableValue::Float(value) => write!(f, "{value}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[test]
    fn should_display_variable_name() {
        let target = ModNutVariableName("varname".to_string());
        let result = format!("{target}");

        assert_that!(result, eq("varname"));
    }

    #[test]
    fn should_display_variable_description() {
        let target = ModNutVariableDescription("description".to_string());
        let result = format!("{target}");

        assert_that!(result, eq("description"));
    }

    #[test]
    fn should_display_variable_description_escaped() {
        let target = ModNutVariableDescription("\"description\"".to_string());
        let result = format!("{target}");

        assert_that!(result, eq("\\\"description\\\""));
    }

    #[test]
    fn should_display_variable_type_read_write() {
        let target = ModNutVariableType::ReadWrite;
        let result = format!("{target}");

        assert_that!(result, eq("RW"));
    }

    #[test]
    fn should_display_variable_type_enumeration() {
        let target = ModNutVariableType::Enumeration;
        let result = format!("{target}");

        assert_that!(result, eq("ENUM"));
    }

    #[test]
    fn should_display_variable_type_string() {
        let target = ModNutVariableType::String(123);
        let result = format!("{target}");

        assert_that!(result, eq("STRING:123"));
    }

    #[test]
    fn should_display_variable_type_range() {
        let target = ModNutVariableType::Range;
        let result = format!("{target}");

        assert_that!(result, eq("RANGE"));
    }

    #[test]
    fn should_display_variable_type_number() {
        let target = ModNutVariableType::Number;
        let result = format!("{target}");

        assert_that!(result, eq("NUMBER"));
    }

    #[test]
    fn should_display_variable_type_enum_variant() {
        let target = ModNutVariableTypeEnumVariant("variant".to_string());
        let result = format!("{target}");

        assert_that!(result, eq("variant"));
    }

    #[test]
    fn should_display_variable_type_enum_variant_escaped() {
        let target = ModNutVariableTypeEnumVariant("\"variant\"".to_string());
        let result = format!("{target}");

        assert_that!(result, eq("\\\"variant\\\""));
    }

    #[test]
    fn should_display_variable_type_range_bound_integer() {
        let target = ModNutVariableTypeRangeBound::Integer(123);
        let result = format!("{target}");

        assert_that!(result, eq("123"));
    }

    #[test]
    fn should_display_variable_type_range_bound_float() {
        let target = ModNutVariableTypeRangeBound::Float(123.4);
        let result = format!("{target}");

        assert_that!(result, eq("123.4"));
    }

    #[test]
    fn should_display_variable_value_string() {
        let target = ModNutVariableValue::String("value".to_string());
        let result = format!("{target}");

        assert_that!(result, eq("value"));
    }

    #[test]
    fn should_display_variable_value_string_escaped() {
        let target = ModNutVariableValue::String("\"value\"".to_string());
        let result = format!("{target}");

        assert_that!(result, eq("\\\"value\\\""));
    }

    #[test]
    fn should_display_variable_value_integer() {
        let target = ModNutVariableValue::Integer(123);
        let result = format!("{target}");

        assert_that!(result, eq("123"));
    }

    #[test]
    fn should_display_variable_value_float() {
        let target = ModNutVariableValue::Float(123.4);
        let result = format!("{target}");

        assert_that!(result, eq("123.4"));
    }
}
