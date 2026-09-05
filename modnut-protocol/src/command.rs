use crate::address::ModNutUpsDeviceName;
use crate::string_escape::escape_string_value;
use crate::variable::{ModNutVariableName, ModNutVariableValue};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum ModNutCommandName {
    // ATTACH
    Attach,
    // DETACH
    Detach,
    // FSD
    ForcedShutdown,
    // GET
    Get,
    // HELP
    Help,
    // INSTCMD
    InstantCommand,
    // LIST
    List,
    // PROTVER
    ProtocolVersion,
    // SET
    Set,
    // VER
    ApplicationVersion,
    // PASSWORD - omitted on purpose
    // PRIMARY - omitted on purpose
    // STARTTLS - omitted on purpose
    // USERNAME - omitted on purpose
}

impl Display for ModNutCommandName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ModNutCommandName::Attach => write!(f, "ATTACH"),
            ModNutCommandName::Detach => write!(f, "DETACH"),
            ModNutCommandName::ForcedShutdown => write!(f, "FSD"),
            ModNutCommandName::Get => write!(f, "GET"),
            ModNutCommandName::Help => write!(f, "HELP"),
            ModNutCommandName::InstantCommand => write!(f, "INSTCMD"),
            ModNutCommandName::List => write!(f, "LIST"),
            ModNutCommandName::ProtocolVersion => write!(f, "PROTVER"),
            ModNutCommandName::Set => write!(f, "SET"),
            ModNutCommandName::ApplicationVersion => write!(f, "VER"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ModNutCommand {
    // ATTACH <upsname>
    Attach(ModNutUpsDeviceName),
    // DETACH
    Detach,
    // FSD <upsname>
    ForcedShutdown(ModNutUpsDeviceName),
    // GET <subcommand>
    Get(ModNutGetCommand),
    // HELP
    Help,
    // INSTCMD <upsname> <cmdname>
    InstantCommand(ModNutUpsDeviceName, ModNutInstantCommandName),
    // LIST <subcommand>
    List(ModNutListCommand),
    // PROTVER
    ProtocolVersion,
    // SET <subcommand>
    Set(ModNutSetCommand),
    // VER
    ApplicationVersion,
    // PASSWORD <password> - omitted on purpose
    // PRIMARY <upsname> - omitted on purpose
    // STARTTLS - omitted on purpose
    // USERNAME <username> - omitted on purpose
}

impl Display for ModNutCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ModNutCommand::Attach(ups_device_name) => write!(f, "ATTACH {ups_device_name}"),
            ModNutCommand::Detach => write!(f, "DETACH"),
            ModNutCommand::ForcedShutdown(ups_device_name) => write!(f, "FSD {ups_device_name}"),
            ModNutCommand::Get(subcommand) => write!(f, "GET {subcommand}"),
            ModNutCommand::Help => write!(f, "HELP"),
            ModNutCommand::InstantCommand(ups_device_name, instant_command_name) => {
                write!(f, "INSTCMD {ups_device_name} {instant_command_name}")
            }
            ModNutCommand::List(subcommand) => write!(f, "LIST {subcommand}"),
            ModNutCommand::ProtocolVersion => write!(f, "PROTVER"),
            ModNutCommand::Set(subcommand) => write!(f, "SET {subcommand}"),
            ModNutCommand::ApplicationVersion => write!(f, "VER"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ModNutGetCommand {
    // CMDDESC <upsname> <cmdname>
    InstantCommandDescription(ModNutUpsDeviceName, ModNutInstantCommandName),
    // DESC <upsname> <varname>
    VariableDescription(ModNutUpsDeviceName, ModNutVariableName),
    // NUMATTACH <upsname>
    ActiveSystems(ModNutUpsDeviceName),
    // TYPE <upsname> <varname>
    VariableType(ModNutUpsDeviceName, ModNutVariableName),
    // UPSDESC <upsname>
    UpsDeviceDescription(ModNutUpsDeviceName),
    // VAR <upsname> <varname>
    VariableValue(ModNutUpsDeviceName, ModNutVariableName),
}

impl Display for ModNutGetCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ModNutGetCommand::InstantCommandDescription(ups_device_name, instant_command_name) => {
                write!(f, "CMDDESC {ups_device_name} {instant_command_name}")
            }
            ModNutGetCommand::VariableDescription(ups_device_name, variable_name) => {
                write!(f, "DESC {ups_device_name} {variable_name}")
            }
            ModNutGetCommand::ActiveSystems(ups_device_name) => {
                write!(f, "NUMATTACH {ups_device_name}")
            }
            ModNutGetCommand::VariableType(ups_device_name, variable_name) => {
                write!(f, "TYPE {ups_device_name} {variable_name}")
            }
            ModNutGetCommand::UpsDeviceDescription(ups_device_name) => {
                write!(f, "UPSDESC {ups_device_name}")
            }
            ModNutGetCommand::VariableValue(ups_device_name, variable_name) => {
                write!(f, "VAR {ups_device_name} {variable_name}")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ModNutListCommand {
    // CLIENT <upsname>
    Clients(ModNutUpsDeviceName),
    // CMD <upsname>
    InstantCommands(ModNutUpsDeviceName),
    // ENUM <upsname> <varname>
    VariableEnumVariants(ModNutUpsDeviceName, ModNutVariableName),
    // RANGE <upsname> <varname>
    VariableRanges(ModNutUpsDeviceName, ModNutVariableName),
    // RW <upsname>
    ReadWriteVariables(ModNutUpsDeviceName),
    // UPS
    UpsDevices,
    // VAR <upsname>
    Variables(ModNutUpsDeviceName),
}

impl Display for ModNutListCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ModNutListCommand::Clients(ups_device_name) => write!(f, "CLIENT {ups_device_name}"),
            ModNutListCommand::InstantCommands(ups_device_name) => {
                write!(f, "CMD {ups_device_name}")
            }
            ModNutListCommand::VariableEnumVariants(ups_device_name, variable_name) => {
                write!(f, "ENUM {ups_device_name} {variable_name}")
            }
            ModNutListCommand::VariableRanges(ups_device_name, variable_name) => {
                write!(f, "RANGE {ups_device_name} {variable_name}")
            }
            ModNutListCommand::ReadWriteVariables(ups_device_name) => {
                write!(f, "RW {ups_device_name}")
            }
            ModNutListCommand::UpsDevices => write!(f, "UPS"),
            ModNutListCommand::Variables(ups_device_name) => write!(f, "VAR {ups_device_name}"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ModNutSetCommand {
    // VAR <upsname> <varname> "<value>"
    VariableValue(ModNutUpsDeviceName, ModNutVariableName, ModNutVariableValue),
}

impl Display for ModNutSetCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ModNutSetCommand::VariableValue(ups_device_name, variable_name, variable_value) => {
                write!(
                    f,
                    "VAR {ups_device_name} {variable_name} \"{variable_value}\""
                )
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModNutInstantCommandName(pub String);

impl Display for ModNutInstantCommandName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct ModNutInstantCommandDescription(pub String);

impl Display for ModNutInstantCommandDescription {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", escape_string_value(self.0.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::address::ModNutUpsDeviceName;
    use googletest::prelude::*;

    #[test]
    fn should_display_command_name_attach() {
        let target = ModNutCommandName::Attach;
        let result = format!("{target}");

        assert_that!(result, eq("ATTACH"));
    }

    #[test]
    fn should_display_command_name_detach() {
        let target = ModNutCommandName::Detach;
        let result = format!("{target}");

        assert_that!(result, eq("DETACH"));
    }

    #[test]
    fn should_display_command_name_forced_shutdown() {
        let target = ModNutCommandName::ForcedShutdown;
        let result = format!("{target}");

        assert_that!(result, eq("FSD"));
    }

    #[test]
    fn should_display_command_name_get() {
        let target = ModNutCommandName::Get;
        let result = format!("{target}");

        assert_that!(result, eq("GET"));
    }

    #[test]
    fn should_display_command_name_help() {
        let target = ModNutCommandName::Help;
        let result = format!("{target}");

        assert_that!(result, eq("HELP"));
    }

    #[test]
    fn should_display_command_name_instant_command() {
        let target = ModNutCommandName::InstantCommand;
        let result = format!("{target}");

        assert_that!(result, eq("INSTCMD"));
    }

    #[test]
    fn should_display_command_name_list() {
        let target = ModNutCommandName::List;
        let result = format!("{target}");

        assert_that!(result, eq("LIST"));
    }

    #[test]
    fn should_display_command_name_protocol_version() {
        let target = ModNutCommandName::ProtocolVersion;
        let result = format!("{target}");

        assert_that!(result, eq("PROTVER"));
    }

    #[test]
    fn should_display_command_name_set() {
        let target = ModNutCommandName::Set;
        let result = format!("{target}");

        assert_that!(result, eq("SET"));
    }

    #[test]
    fn should_display_command_name_application_version() {
        let target = ModNutCommandName::ApplicationVersion;
        let result = format!("{target}");

        assert_that!(result, eq("VER"));
    }

    #[test]
    fn should_display_command_attach() {
        let target = ModNutCommand::Attach(ModNutUpsDeviceName("upsname".to_string()));
        let result = format!("{target}");

        assert_that!(result, eq("ATTACH upsname"));
    }

    #[test]
    fn should_display_command_detach() {
        let target = ModNutCommand::Detach;
        let result = format!("{target}");

        assert_that!(result, eq("DETACH"));
    }

    #[test]
    fn should_display_command_forced_shutdown() {
        let target = ModNutCommand::ForcedShutdown(ModNutUpsDeviceName("upsname".to_string()));
        let result = format!("{target}");

        assert_that!(result, eq("FSD upsname"));
    }

    #[test]
    fn should_display_command_get_instant_command_description() {
        let target = ModNutCommand::Get(ModNutGetCommand::InstantCommandDescription(
            ModNutUpsDeviceName("upsname".to_string()),
            ModNutInstantCommandName("cmdname".to_string()),
        ));
        let result = format!("{target}");

        assert_that!(result, eq("GET CMDDESC upsname cmdname"));
    }

    #[test]
    fn should_display_command_get_variable_description() {
        let target = ModNutCommand::Get(ModNutGetCommand::VariableDescription(
            ModNutUpsDeviceName("upsname".to_string()),
            ModNutVariableName("varname".to_string()),
        ));
        let result = format!("{target}");

        assert_that!(result, eq("GET DESC upsname varname"));
    }

    #[test]
    fn should_display_command_get_active_systems() {
        let target = ModNutCommand::Get(ModNutGetCommand::ActiveSystems(ModNutUpsDeviceName(
            "upsname".to_string(),
        )));
        let result = format!("{target}");

        assert_that!(result, eq("GET NUMATTACH upsname"));
    }

    #[test]
    fn should_display_command_get_variable_type() {
        let target = ModNutCommand::Get(ModNutGetCommand::VariableType(
            ModNutUpsDeviceName("upsname".to_string()),
            ModNutVariableName("varname".to_string()),
        ));
        let result = format!("{target}");

        assert_that!(result, eq("GET TYPE upsname varname"));
    }

    #[test]
    fn should_display_command_get_ups_device_description() {
        let target = ModNutCommand::Get(ModNutGetCommand::UpsDeviceDescription(
            ModNutUpsDeviceName("upsname".to_string()),
        ));
        let result = format!("{target}");

        assert_that!(result, eq("GET UPSDESC upsname"));
    }

    #[test]
    fn should_display_command_get_variable_value() {
        let target = ModNutCommand::Get(ModNutGetCommand::VariableValue(
            ModNutUpsDeviceName("upsname".to_string()),
            ModNutVariableName("varname".to_string()),
        ));
        let result = format!("{target}");

        assert_that!(result, eq("GET VAR upsname varname"));
    }

    #[test]
    fn should_display_command_help() {
        let target = ModNutCommand::Help;
        let result = format!("{target}");

        assert_that!(result, eq("HELP"));
    }

    #[test]
    fn should_display_command_instant_command() {
        let target = ModNutCommand::InstantCommand(
            ModNutUpsDeviceName("upsname".to_string()),
            ModNutInstantCommandName("cmdname".to_string()),
        );
        let result = format!("{target}");

        assert_that!(result, eq("INSTCMD upsname cmdname"));
    }

    #[test]
    fn should_display_command_list_clients() {
        let target = ModNutCommand::List(ModNutListCommand::Clients(ModNutUpsDeviceName(
            "upsname".to_string(),
        )));
        let result = format!("{target}");

        assert_that!(result, eq("LIST CLIENT upsname"));
    }

    #[test]
    fn should_display_command_list_instant_commands() {
        let target = ModNutCommand::List(ModNutListCommand::InstantCommands(ModNutUpsDeviceName(
            "upsname".to_string(),
        )));
        let result = format!("{target}");

        assert_that!(result, eq("LIST CMD upsname"));
    }

    #[test]
    fn should_display_command_list_variable_enum_variants() {
        let target = ModNutCommand::List(ModNutListCommand::VariableEnumVariants(
            ModNutUpsDeviceName("upsname".to_string()),
            ModNutVariableName("varname".to_string()),
        ));
        let result = format!("{target}");

        assert_that!(result, eq("LIST ENUM upsname varname"));
    }

    #[test]
    fn should_display_command_list_variable_range() {
        let target = ModNutCommand::List(ModNutListCommand::VariableRanges(
            ModNutUpsDeviceName("upsname".to_string()),
            ModNutVariableName("varname".to_string()),
        ));
        let result = format!("{target}");

        assert_that!(result, eq("LIST RANGE upsname varname"));
    }

    #[test]
    fn should_display_command_list_read_write_variables() {
        let target = ModNutCommand::List(ModNutListCommand::ReadWriteVariables(
            ModNutUpsDeviceName("upsname".to_string()),
        ));
        let result = format!("{target}");

        assert_that!(result, eq("LIST RW upsname"));
    }

    #[test]
    fn should_display_command_list_ups_devices() {
        let target = ModNutCommand::List(ModNutListCommand::UpsDevices);
        let result = format!("{target}");

        assert_that!(result, eq("LIST UPS"));
    }

    #[test]
    fn should_display_command_list_variables() {
        let target = ModNutCommand::List(ModNutListCommand::Variables(ModNutUpsDeviceName(
            "upsname".to_string(),
        )));
        let result = format!("{target}");

        assert_that!(result, eq("LIST VAR upsname"));
    }

    #[test]
    fn should_display_command_protocol_version() {
        let target = ModNutCommand::ProtocolVersion;
        let result = format!("{target}");

        assert_that!(result, eq("PROTVER"));
    }

    #[test]
    fn should_display_command_set_variable_value_string() {
        let target = ModNutCommand::Set(ModNutSetCommand::VariableValue(
            ModNutUpsDeviceName("upsname".to_string()),
            ModNutVariableName("varname".to_string()),
            ModNutVariableValue::String("value".to_string()),
        ));
        let result = format!("{target}");

        assert_that!(result, eq("SET VAR upsname varname \"value\""));
    }

    #[test]
    fn should_display_command_set_variable_value_string_escaped() {
        let target = ModNutCommand::Set(ModNutSetCommand::VariableValue(
            ModNutUpsDeviceName("upsname".to_string()),
            ModNutVariableName("varname".to_string()),
            ModNutVariableValue::String("\"value\"".to_string()),
        ));
        let result = format!("{target}");

        assert_that!(result, eq("SET VAR upsname varname \"\\\"value\\\"\""));
    }

    #[test]
    fn should_display_command_set_variable_value_integer() {
        let target = ModNutCommand::Set(ModNutSetCommand::VariableValue(
            ModNutUpsDeviceName("upsname".to_string()),
            ModNutVariableName("varname".to_string()),
            ModNutVariableValue::Integer(123),
        ));
        let result = format!("{target}");

        assert_that!(result, eq("SET VAR upsname varname \"123\""));
    }

    #[test]
    fn should_display_command_set_variable_value_float() {
        let target = ModNutCommand::Set(ModNutSetCommand::VariableValue(
            ModNutUpsDeviceName("upsname".to_string()),
            ModNutVariableName("varname".to_string()),
            ModNutVariableValue::Float(123.4),
        ));
        let result = format!("{target}");

        assert_that!(result, eq("SET VAR upsname varname \"123.4\""));
    }

    #[test]
    fn should_display_command_application_version() {
        let target = ModNutCommand::ApplicationVersion;
        let result = format!("{target}");

        assert_that!(result, eq("VER"));
    }

    #[test]
    fn should_display_instant_command_name() {
        let target = ModNutInstantCommandName("cmdname".to_string());
        let result = format!("{target}");

        assert_that!(result, eq("cmdname"));
    }

    #[test]
    fn should_display_instant_command_description() {
        let target = ModNutInstantCommandDescription("description".to_string());
        let result = format!("{target}");

        assert_that!(result, eq("description"));
    }

    #[test]
    fn should_display_instant_command_description_escaped() {
        let target = ModNutInstantCommandDescription("\"description\"".to_string());
        let result = format!("{target}");

        assert_that!(result, eq("\\\"description\\\""));
    }
}
