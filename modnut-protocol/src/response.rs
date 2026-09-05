use crate::address::{
    ModNutUpsDeviceActiveSystems, ModNutUpsDeviceDescription, ModNutUpsDeviceName,
};
use crate::command::{
    ModNutCommandName, ModNutInstantCommandDescription, ModNutInstantCommandName,
};
use crate::variable::{
    ModNutVariableDescription, ModNutVariableName, ModNutVariableType,
    ModNutVariableTypeEnumVariant, ModNutVariableTypeRangeBound, ModNutVariableValue,
};
use std::fmt::{Display, Formatter};
use std::net::IpAddr;

#[derive(Debug, Clone)]
pub enum ModNutResponse {
    // OK <message>
    Ok(Option<ModNutResponseOkMessage>),

    // <response>
    Get(ModNutGetResponse),

    // <response>
    Help(ModNutHelpResponse),

    // <response>
    List(ModNutListResponse),

    // <version>
    Version(ModNutResponseVersion),
}

impl Display for ModNutResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ModNutResponse::Ok(Some(message)) => write!(f, "OK {message}"),
            ModNutResponse::Ok(None) => write!(f, "OK"),
            ModNutResponse::Get(response) => write!(f, "{response}"),
            ModNutResponse::Help(response) => write!(f, "{response}"),
            ModNutResponse::List(response) => write!(f, "{response}"),
            ModNutResponse::Version(version) => write!(f, "{version}"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ModNutResponseOkMessage {
    // Goodbye
    Goodbye,
    // FSD-SET
    ForcedShutdownInitiated,
    // <message>
    Other(String),
}

impl Display for ModNutResponseOkMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ModNutResponseOkMessage::Goodbye => write!(f, "Goodbye"),
            ModNutResponseOkMessage::ForcedShutdownInitiated => write!(f, "FSD-SET"),
            ModNutResponseOkMessage::Other(message) => write!(f, "{message}"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ModNutGetResponse {
    // CMDDESC <upsname> <cmdname> "<description>"
    InstantCommandDescription(
        ModNutUpsDeviceName,
        ModNutInstantCommandName,
        ModNutInstantCommandDescription,
    ),

    // DESC <upsname> <varname> "<description>"
    VariableDescription(
        ModNutUpsDeviceName,
        ModNutVariableName,
        ModNutVariableDescription,
    ),

    // NUMATTACH <upsname> <value>
    ActiveSystems(ModNutUpsDeviceName, ModNutUpsDeviceActiveSystems),

    // TYPE <upsname> <varname> <type>...
    VariableType(
        ModNutUpsDeviceName,
        ModNutVariableName,
        Vec<ModNutVariableType>,
    ),

    // UPSDESC <upsname> "<description>"
    UpsDeviceDescription(ModNutUpsDeviceName, ModNutUpsDeviceDescription),

    // VAR <upsname> <varname> "<value>"
    VariableValue(ModNutUpsDeviceName, ModNutVariableName, ModNutVariableValue),
}

impl Display for ModNutGetResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ModNutGetResponse::InstantCommandDescription(
                ups_device_name,
                instant_command_name,
                instant_command_description,
            ) => {
                write!(
                    f,
                    "CMDDESC {ups_device_name} {instant_command_name} \"{instant_command_description}\""
                )
            }
            ModNutGetResponse::VariableDescription(
                ups_device_name,
                variable_name,
                variable_description,
            ) => {
                write!(
                    f,
                    "DESC {ups_device_name} {variable_name} \"{variable_description}\""
                )
            }
            ModNutGetResponse::ActiveSystems(ups_device_name, active_systems) => {
                write!(f, "NUMATTACH {ups_device_name} {active_systems}")
            }
            ModNutGetResponse::VariableType(ups_device_name, variable_name, variable_types) => {
                let variable_types = variable_types
                    .iter()
                    .fold("".to_string(), |accumulator, variable_type| {
                        format!("{accumulator} {variable_type}")
                    });

                write!(f, "TYPE {ups_device_name} {variable_name} {variable_types}")
            }
            ModNutGetResponse::UpsDeviceDescription(ups_device_name, ups_device_description) => {
                write!(f, "UPSDESC {ups_device_name} \"{ups_device_description}\"")
            }
            ModNutGetResponse::VariableValue(ups_device_name, variable_name, variable_value) => {
                write!(
                    f,
                    "VAR {ups_device_name} {variable_name} \"{variable_value}\""
                )
            }
        }
    }
}

// Commands: <cmdname>...
#[derive(Debug, Clone)]
pub struct ModNutHelpResponse(Vec<ModNutCommandName>);

impl Display for ModNutHelpResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Commands:")?;

        for command_name in self.0.iter() {
            write!(f, " {command_name}")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum ModNutListResponse {
    // BEGIN LIST CLIENT <upsname>
    // CLIENT <upsname> <client_IP_address>
    // ...
    // END LIST CLIENT <upsname>
    Clients(ModNutUpsDeviceName, Vec<ModNutListResponseClient>),

    // BEGIN LIST CMD <upsname>
    // CMD <upsname> <cmdname>
    // ...
    // END LIST CMD <upsname>
    InstantCommands(ModNutUpsDeviceName, Vec<ModNutInstantCommandName>),

    // BEGIN LIST ENUM <upsname> <varname>
    // LIST ENUM <upsname> <varname> "<value>"
    // ...
    // END LIST ENUM <upsname> <varname>
    VariableEnumVariants(
        ModNutUpsDeviceName,
        ModNutVariableName,
        Vec<ModNutVariableTypeEnumVariant>,
    ),

    // BEGIN LIST RANGE <upsname> <varname>
    // LIST RANGE <upsname> <varname> "<min>" "<max>"
    // ...
    // END LIST RANGE <upsname> <varname>
    VariableRanges(
        ModNutUpsDeviceName,
        ModNutVariableName,
        Vec<(ModNutVariableTypeRangeBound, ModNutVariableTypeRangeBound)>,
    ),

    // BEGIN LIST RW <upsname>
    // LIST RW <upsname> <varname> "<value>"
    // ...
    // END LIST RW <upsname>
    ReadWriteVariables(
        ModNutUpsDeviceName,
        Vec<(ModNutVariableName, ModNutVariableValue)>,
    ),

    // BEGIN LIST UPS
    // LIST UPS <upsname> "<description>"
    // ...
    // END LIST UPS
    UpsDevices(Vec<(ModNutUpsDeviceName, ModNutUpsDeviceDescription)>),

    // BEGIN LIST VAR <upsname>
    // LIST VAR <upsname> <varname> "<value>"
    // ...
    // END LIST VAR <upsname>
    Variables(
        ModNutUpsDeviceName,
        Vec<(ModNutVariableName, ModNutVariableValue)>,
    ),
}

impl Display for ModNutListResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let subcommand = match self {
            ModNutListResponse::Clients(ups_device_name, _) => format!("CLIENT {ups_device_name}"),
            ModNutListResponse::InstantCommands(ups_device_name, _) => {
                format!("CMD {ups_device_name}")
            }
            ModNutListResponse::VariableEnumVariants(ups_device_name, variable_name, _) => {
                format!("ENUM {ups_device_name} {variable_name}")
            }
            ModNutListResponse::VariableRanges(ups_device_name, variable_name, _) => {
                format!("RANGE {ups_device_name} {variable_name}")
            }
            ModNutListResponse::ReadWriteVariables(ups_device_name, _) => {
                format!("RW {ups_device_name}")
            }
            ModNutListResponse::UpsDevices(_) => "UPS".to_string(),
            ModNutListResponse::Variables(ups_device_name, _) => format!("VAR {ups_device_name}"),
        };

        let values: Vec<String> = match self {
            ModNutListResponse::Clients(_, clients) => {
                clients.iter().map(|client| format!("{client}")).collect()
            }
            ModNutListResponse::InstantCommands(_, instant_command_names) => instant_command_names
                .iter()
                .map(|instant_command_name| format!("{instant_command_name}"))
                .collect(),
            ModNutListResponse::VariableEnumVariants(_, _, enum_variants) => enum_variants
                .iter()
                .map(|enum_variant| format!("\"{enum_variant}\""))
                .collect(),
            ModNutListResponse::VariableRanges(_, _, variable_ranges) => variable_ranges
                .iter()
                .map(|(variable_range_min_bound, variable_range_max_bound)| {
                    format!("\"{variable_range_min_bound}\" \"{variable_range_max_bound}\"")
                })
                .collect(),
            ModNutListResponse::ReadWriteVariables(_, read_write_variables) => read_write_variables
                .iter()
                .map(|(variable_name, variable_value)| {
                    format!("{variable_name} \"{variable_value}\"")
                })
                .collect(),
            ModNutListResponse::UpsDevices(ups_device_names_and_descriptions) => {
                ups_device_names_and_descriptions
                    .iter()
                    .map(|(ups_device_name, ups_device_description)| {
                        format!("{ups_device_name} \"{ups_device_description}\"")
                    })
                    .collect()
            }
            ModNutListResponse::Variables(_, variables) => variables
                .iter()
                .map(|(variable_name, variable_value)| {
                    format!("{variable_name} \"{variable_value}\"")
                })
                .collect(),
        };

        writeln!(f, "BEGIN LIST {subcommand}")?;
        for value in values.into_iter() {
            writeln!(f, "{subcommand} {value}")?;
        }
        writeln!(f, "END LIST {subcommand}")?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ModNutListResponseClient(pub IpAddr);

impl Display for ModNutListResponseClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct ModNutResponseVersion(pub String);

impl Display for ModNutResponseVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
