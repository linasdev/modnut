use crate::usb_hid::report::error::UsbHidReportError;
use crate::usb_hid::report::field::{
    UsbHidReportArrayField, UsbHidReportField, UsbHidReportPaddingField, UsbHidReportVariableField,
};
use crate::usb_hid::report::{UsbHidReport, UsbHidReportCollection, UsbHidReportDescriptor};
use hid_types::id::{CollectionType, GlobalItem};
use hid_types::item;
use hid_types::item::usage::ExtendedUsage;
use std::collections::{BTreeSet, HashMap};
use std::sync::{Arc, Mutex};

macro_rules! current_global_state {
    ($self:ident, $state_type:ident) => {
        $self
            .global_state
            .get(&GlobalItem::$state_type.into())
            .map(|global_item| {
                let item::Global::$state_type(value) = global_item else {
                    panic!(
                        "Mismatching item types: expected Global::{}, found {global_item:?}",
                        stringify!($state_type),
                    );
                };

                *value
            })
    };
}

pub struct UsbHidReportDescriptorBuilder {
    global_state: HashMap<u8, item::Global>,
    global_state_stack: Vec<HashMap<u8, item::Global>>,
    local_state: Vec<item::Local>,

    input_report_fields: HashMap<Option<u8>, Vec<UsbHidReportField>>,
    output_report_fields: HashMap<Option<u8>, Vec<UsbHidReportField>>,
    feature_report_fields: HashMap<Option<u8>, Vec<UsbHidReportField>>,
    root_collections: Vec<UsbHidReportCollectionBuilder>,
    open_collections: Vec<UsbHidReportCollectionBuilder>,
}

#[derive(Debug, Clone)]
pub struct UsbHidReportCollectionBuilder(Arc<Mutex<UsbHidReportCollectionBuilderInner>>);

#[derive(Debug)]
pub struct UsbHidReportCollectionBuilderInner {
    usage: ExtendedUsage,
    collection_type: CollectionType,
    input_report_ids: BTreeSet<Option<u8>>,
    output_report_ids: BTreeSet<Option<u8>>,
    feature_report_ids: BTreeSet<Option<u8>>,
    child_collections: Vec<UsbHidReportCollectionBuilder>,
}

impl UsbHidReportDescriptorBuilder {
    pub fn new() -> Self {
        Self {
            global_state: HashMap::new(),
            global_state_stack: vec![],
            local_state: vec![],

            input_report_fields: HashMap::new(),
            output_report_fields: HashMap::new(),
            feature_report_fields: HashMap::new(),
            root_collections: vec![],
            open_collections: vec![],
        }
    }

    pub fn build(
        mut self,
        report_descriptor_items: impl IntoIterator<Item = item::Item>,
    ) -> Result<UsbHidReportDescriptor, UsbHidReportError> {
        for report_descriptor_item in report_descriptor_items.into_iter() {
            match report_descriptor_item {
                item::Item::Main(main_item) => self.parse_main_item(main_item)?,
                item::Item::Global(global_item) => self.parse_global_item(global_item)?,
                item::Item::Local(local_item) => self.parse_local_item(local_item),
                _ => {}
            }
        }

        let input_reports = self
            .input_report_fields
            .into_iter()
            .map(|(report_id, fields)| UsbHidReport::new(report_id, fields))
            .collect::<Vec<_>>();

        let output_reports = self
            .output_report_fields
            .into_iter()
            .map(|(report_id, fields)| UsbHidReport::new(report_id, fields))
            .collect::<Vec<_>>();

        let feature_reports = self
            .feature_report_fields
            .into_iter()
            .map(|(report_id, fields)| UsbHidReport::new(report_id, fields))
            .collect::<Vec<_>>();

        let root_collections = self
            .root_collections
            .into_iter()
            .map(|collection_builder| {
                collection_builder.build(&input_reports, &output_reports, &feature_reports)
            })
            .collect();

        if input_reports.len() > 1 {
            if input_reports
                .iter()
                .any(|input_report| input_report.report_id().is_none())
            {
                return Err(UsbHidReportError::ReportIdMissing);
            }
        }

        if output_reports.len() > 1 {
            if output_reports
                .iter()
                .any(|output_report| output_report.report_id().is_none())
            {
                return Err(UsbHidReportError::ReportIdMissing);
            }
        }

        if feature_reports.len() > 1 {
            if feature_reports
                .iter()
                .any(|feature_report| feature_report.report_id().is_none())
            {
                return Err(UsbHidReportError::ReportIdMissing);
            }
        }

        Ok(UsbHidReportDescriptor { root_collections })
    }

    fn parse_main_item(&mut self, main_item: item::Main) -> Result<(), UsbHidReportError> {
        let report_id = current_global_state!(self, ReportId);

        let (field_flags, report_fields) = match main_item {
            item::Main::Input(input_flags) => (
                Some(input_flags.0),
                Some(&*self.input_report_fields.entry(report_id).or_default()),
            ),
            item::Main::Output(output_flags) => (
                Some(output_flags.0),
                Some(&*self.output_report_fields.entry(report_id).or_default()),
            ),
            item::Main::Feature(feature_flags) => (
                Some(feature_flags.0),
                Some(&*self.feature_report_fields.entry(report_id).or_default()),
            ),
            item::Main::Collection(collection_type) => {
                let Some(usage) = self.current_usages()?.first().copied() else {
                    return Err(UsbHidReportError::UsageMissing);
                };

                let collection = UsbHidReportCollectionBuilder::new(usage, collection_type);

                if let Some(last_collection) = self.open_collections.last_mut() {
                    last_collection.add_child_collection(collection.clone());
                }

                self.open_collections.push(collection);

                (None, None)
            }
            item::Main::EndCollection => {
                if let Some(collection) = self.open_collections.pop() {
                    if self.open_collections.is_empty() {
                        self.root_collections.push(collection);
                    }
                } else {
                    return Err(UsbHidReportError::UnexpectedEndCollection);
                }

                (None, None)
            }
            item::Main::Reserved(_, _) => (None, None),
        };

        if let Some(field_flags) = field_flags
            && let Some(report_fields) = report_fields
        {
            let current_field_offset = report_fields
                .iter()
                .map(|field| field.bits().end)
                .max()
                .unwrap_or(8);

            let usages = self.current_usages()?;
            let report_size = self.current_report_size()?;
            let report_count = self.current_report_count()?;
            let designator_indices = self.current_designator_indices()?;
            let string_indices = self.current_string_indices()?;

            let mut fields = (0..report_count)
                .map(|index| {
                    let start_offset = current_field_offset + report_size * index;
                    let end_offset = current_field_offset + report_size * (index + 1);
                    let bits = start_offset..end_offset;

                    if usages.is_empty() {
                        UsbHidReportField::Padding(Arc::new(UsbHidReportPaddingField { bits }))
                    } else if field_flags.is_variable() {
                        let usage = usages
                            .get(index as usize)
                            .or_else(|| usages.last())
                            .copied()
                            .unwrap();
                        let designator_index = designator_indices
                            .get(index as usize)
                            .or_else(|| designator_indices.last())
                            .copied();
                        let string_index = string_indices
                            .get(index as usize)
                            .or_else(|| string_indices.last())
                            .copied();

                        UsbHidReportField::Variable(Arc::new(UsbHidReportVariableField {
                            is_constant: field_flags.is_constant(),
                            report_id,
                            usage,
                            designator_index,
                            string_index,
                            bits: start_offset..end_offset,
                            logical_minimum: current_global_state!(self, LogicalMinimum),
                            logical_maximum: current_global_state!(self, LogicalMaximum),
                            physical_minimum: current_global_state!(self, PhysicalMinimum),
                            physical_maximum: current_global_state!(self, PhysicalMaximum),
                            unit_exponent: current_global_state!(self, UnitExponent),
                            unit: current_global_state!(self, Unit),
                        }))
                    } else {
                        UsbHidReportField::Array(Arc::new(UsbHidReportArrayField {
                            is_constant: field_flags.is_constant(),
                            report_id,
                            usages: usages.clone(),
                            designator_indices: designator_indices.clone(),
                            string_indices: string_indices.clone(),
                            bits: start_offset..end_offset,
                            logical_minimum: current_global_state!(self, LogicalMinimum),
                            logical_maximum: current_global_state!(self, LogicalMaximum),
                        }))
                    }
                })
                .collect();

            let Some(last_collection) = self.open_collections.last_mut() else {
                return Err(UsbHidReportError::ReportOutsideCollection);
            };

            match main_item {
                item::Main::Input(_) => {
                    last_collection.add_input_report_id(report_id);
                    self.input_report_fields
                        .entry(report_id)
                        .or_default()
                        .append(&mut fields)
                }
                item::Main::Output(_) => {
                    last_collection.add_output_report_id(report_id);
                    self.output_report_fields
                        .entry(report_id)
                        .or_default()
                        .append(&mut fields)
                }
                item::Main::Feature(_) => {
                    last_collection.add_feature_report_id(report_id);
                    self.feature_report_fields
                        .entry(report_id)
                        .or_default()
                        .append(&mut fields)
                }
                _ => {}
            };
        }

        self.local_state.clear();

        Ok(())
    }

    fn parse_global_item(&mut self, global_item: item::Global) -> Result<(), UsbHidReportError> {
        match global_item {
            item::Global::UsagePage(_) => {
                self.global_state
                    .insert(GlobalItem::UsagePage.into(), global_item);
            }
            item::Global::LogicalMinimum(_) => {
                self.global_state
                    .insert(GlobalItem::LogicalMinimum.into(), global_item);
            }
            item::Global::LogicalMaximum(_) => {
                self.global_state
                    .insert(GlobalItem::LogicalMaximum.into(), global_item);
            }
            item::Global::PhysicalMinimum(_) => {
                self.global_state
                    .insert(GlobalItem::PhysicalMinimum.into(), global_item);
            }
            item::Global::PhysicalMaximum(_) => {
                self.global_state
                    .insert(GlobalItem::PhysicalMaximum.into(), global_item);
            }
            item::Global::UnitExponent(_) => {
                self.global_state
                    .insert(GlobalItem::UnitExponent.into(), global_item);
            }
            item::Global::Unit(_) => {
                self.global_state
                    .insert(GlobalItem::Unit.into(), global_item);
            }
            item::Global::ReportSize(_) => {
                self.global_state
                    .insert(GlobalItem::ReportSize.into(), global_item);
            }
            item::Global::ReportId(_) => {
                self.global_state
                    .insert(GlobalItem::ReportId.into(), global_item);
            }
            item::Global::ReportCount(_) => {
                self.global_state
                    .insert(GlobalItem::ReportCount.into(), global_item);
            }
            item::Global::Reserved(item_tag, _) => {
                self.global_state.insert(item_tag, global_item);
            }
            item::Global::Push => {
                self.global_state_stack.push(self.global_state.clone());
            }
            item::Global::Pop => {
                if let Some(global_state) = self.global_state_stack.pop() {
                    self.global_state = global_state;
                } else {
                    return Err(UsbHidReportError::PopBeforePush);
                }
            }
        }

        Ok(())
    }

    fn parse_local_item(&mut self, local_item: item::Local) {
        self.local_state.push(local_item);
    }

    fn current_report_size(&self) -> Result<u32, UsbHidReportError> {
        current_global_state!(self, ReportSize).ok_or(UsbHidReportError::ReportSizeMissing)
    }

    fn current_report_count(&self) -> Result<u32, UsbHidReportError> {
        current_global_state!(self, ReportCount).ok_or(UsbHidReportError::ReportCountMissing)
    }

    fn current_usages(&self) -> Result<Vec<ExtendedUsage>, UsbHidReportError> {
        let mut current_usages = vec![];
        let mut current_usage_minimum = None;
        let current_usage_page = current_global_state!(self, UsagePage);

        for local_item in self.local_state.iter() {
            match local_item {
                item::Local::Usage(usage) => {
                    if let Some(usage_page) = current_usage_page {
                        current_usages.push(ExtendedUsage::new(usage_page, usage.to_integer()))
                    } else {
                        return Err(UsbHidReportError::UsagePageMissing);
                    }
                }
                item::Local::ExtendedUsage(extended_usage) => current_usages.push(*extended_usage),
                item::Local::UsageMinimum(usage_minimum) => {
                    if let Some(_) = current_usage_minimum.replace(usage_minimum) {
                        return Err(UsbHidReportError::InvalidUsageRange);
                    }
                }
                item::Local::UsageMaximum(usage_maximum) => {
                    if let Some(usage_minimum) = current_usage_minimum.take() {
                        for usage_id in *usage_minimum..=*usage_maximum {
                            if usage_id > u16::MAX as u32 {
                                current_usages.push(ExtendedUsage::from_u32(usage_id));
                            } else if let Some(usage_page) = current_usage_page {
                                current_usages
                                    .push(ExtendedUsage::new(usage_page, usage_id as u16));
                            } else {
                                return Err(UsbHidReportError::UsagePageMissing);
                            }
                        }
                    } else {
                        return Err(UsbHidReportError::InvalidUsageRange);
                    }
                }
                _ => {}
            }
        }

        Ok(current_usages)
    }

    fn current_designator_indices(&self) -> Result<Vec<u32>, UsbHidReportError> {
        let mut current_designator_indices = vec![];
        let mut current_designator_index_minimum = None;

        for local_item in self.local_state.iter() {
            match local_item {
                item::Local::DesignatorIndex(designator_index) => {
                    current_designator_indices.push(*designator_index)
                }
                item::Local::DesignatorMinimum(designator_index_minimum) => {
                    if let Some(_) =
                        current_designator_index_minimum.replace(designator_index_minimum)
                    {
                        return Err(UsbHidReportError::InvalidDesignatorIndexRange);
                    }
                }
                item::Local::DesignatorMaximum(designator_index_maximum) => {
                    if let Some(designator_index_minimum) = current_designator_index_minimum.take()
                    {
                        for designator_index in
                            *designator_index_minimum..=*designator_index_maximum
                        {
                            current_designator_indices.push(designator_index);
                        }
                    } else {
                        return Err(UsbHidReportError::InvalidDesignatorIndexRange);
                    }
                }
                _ => {}
            }
        }

        Ok(current_designator_indices)
    }

    fn current_string_indices(&self) -> Result<Vec<u32>, UsbHidReportError> {
        let mut current_string_indices = vec![];
        let mut current_string_index_minimum = None;

        for local_item in self.local_state.iter() {
            match local_item {
                item::Local::StringIndex(string_index) => {
                    current_string_indices.push(*string_index)
                }
                item::Local::StringMinimum(string_index_minimum) => {
                    if let Some(_) = current_string_index_minimum.replace(string_index_minimum) {
                        return Err(UsbHidReportError::InvalidStringIndexRange);
                    }
                }
                item::Local::StringMaximum(string_index_maximum) => {
                    if let Some(string_index_minimum) = current_string_index_minimum.take() {
                        for string_index in *string_index_minimum..=*string_index_maximum {
                            current_string_indices.push(string_index);
                        }
                    } else {
                        return Err(UsbHidReportError::InvalidStringIndexRange);
                    }
                }
                _ => {}
            }
        }

        Ok(current_string_indices)
    }
}

impl UsbHidReportCollectionBuilder {
    pub fn new(usage: ExtendedUsage, collection_type: CollectionType) -> Self {
        let inner = UsbHidReportCollectionBuilderInner {
            usage,
            collection_type,
            input_report_ids: BTreeSet::new(),
            output_report_ids: BTreeSet::new(),
            feature_report_ids: BTreeSet::new(),
            child_collections: vec![],
        };

        Self(Arc::new(Mutex::new(inner)))
    }

    pub fn add_input_report_id(&self, report_id: Option<u8>) {
        self.0.lock().unwrap().input_report_ids.insert(report_id);
    }

    pub fn add_output_report_id(&self, report_id: Option<u8>) {
        self.0.lock().unwrap().output_report_ids.insert(report_id);
    }

    pub fn add_feature_report_id(&self, report_id: Option<u8>) {
        self.0.lock().unwrap().feature_report_ids.insert(report_id);
    }

    pub fn add_child_collection(&self, collection: UsbHidReportCollectionBuilder) {
        self.0.lock().unwrap().child_collections.push(collection);
    }

    pub fn build(
        &self,
        all_input_reports: &Vec<UsbHidReport>,
        all_output_reports: &Vec<UsbHidReport>,
        all_feature_reports: &Vec<UsbHidReport>,
    ) -> UsbHidReportCollection {
        let inner = self.0.lock().unwrap();

        let child_collections = inner
            .child_collections
            .iter()
            .map(|collection_builder| {
                collection_builder.build(all_input_reports, all_output_reports, all_feature_reports)
            })
            .collect();

        let input_reports = inner
            .input_report_ids
            .iter()
            .map(|&report_id| {
                all_input_reports
                    .iter()
                    .find(|&report| report.report_id() == report_id)
                    .expect("input report not found")
                    .clone()
            })
            .collect();

        let output_reports = inner
            .output_report_ids
            .iter()
            .map(|&report_id| {
                all_output_reports
                    .iter()
                    .find(|&report| report.report_id() == report_id)
                    .expect("output report not found")
                    .clone()
            })
            .collect();

        let feature_reports = inner
            .feature_report_ids
            .iter()
            .map(|&report_id| {
                all_feature_reports
                    .iter()
                    .find(|&report| report.report_id() == report_id)
                    .expect("feature report not found")
                    .clone()
            })
            .collect();

        UsbHidReportCollection::new(
            inner.usage,
            inner.collection_type,
            input_reports,
            output_reports,
            feature_reports,
            child_collections,
        )
    }
}
