use crate::string_escape::escape_string_value;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct ModNutUpsDeviceName(pub String);

impl Display for ModNutUpsDeviceName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct ModNutUpsDeviceDescription(pub String);

impl Display for ModNutUpsDeviceDescription {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", escape_string_value(self.0.as_str()))
    }
}

#[derive(Debug, Clone)]
pub struct ModNutUpsDeviceActiveSystems(pub u64);

impl Display for ModNutUpsDeviceActiveSystems {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct ModNutUpsDeviceAddress {
    name: ModNutUpsDeviceName,
    host_and_port: Option<(String, Option<u16>)>,
}

impl Display for ModNutUpsDeviceAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some((host, Some(port))) = self.host_and_port.as_ref() {
            write!(f, "{}@{host}:{port}", self.name)
        } else if let Some((host, None)) = self.host_and_port.as_ref() {
            write!(f, "{}@{host}", self.name)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[test]
    fn should_display_ups_device_name() {
        let target = ModNutUpsDeviceName("upsname".to_string());
        let result = format!("{target}");

        assert_that!(result, eq("upsname"));
    }

    #[test]
    fn should_display_ups_device_description() {
        let target = ModNutUpsDeviceDescription("description".to_string());
        let result = format!("{target}");

        assert_that!(result, eq("description"));
    }

    #[test]
    fn should_display_ups_device_description_escaped() {
        let target = ModNutUpsDeviceDescription("\"description\"".to_string());
        let result = format!("{target}");

        assert_that!(result, eq("\\\"description\\\""));
    }

    #[test]
    fn should_display_ups_device_active_systems() {
        let target = ModNutUpsDeviceActiveSystems(123);
        let result = format!("{target}");

        assert_that!(result, eq("123"));
    }

    #[test]
    fn should_display_ups_device_address_with_host_and_port() {
        let target = ModNutUpsDeviceAddress {
            name: ModNutUpsDeviceName("upsname".to_string()),
            host_and_port: Some(("127.0.0.1".to_string(), Some(3494))),
        };
        let result = format!("{target}");

        assert_that!(result, eq("upsname@127.0.0.1:3494"));
    }

    #[test]
    fn should_display_ups_device_address_with_host() {
        let target = ModNutUpsDeviceAddress {
            name: ModNutUpsDeviceName("upsname".to_string()),
            host_and_port: Some(("127.0.0.1".to_string(), None)),
        };
        let result = format!("{target}");

        assert_that!(result, eq("upsname@127.0.0.1"));
    }

    #[test]
    fn should_display_ups_device_address() {
        let target = ModNutUpsDeviceAddress {
            name: ModNutUpsDeviceName("upsname".to_string()),
            host_and_port: None,
        };
        let result = format!("{target}");

        assert_that!(result, eq("upsname"));
    }
}
