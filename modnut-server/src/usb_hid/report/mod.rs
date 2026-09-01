use crate::usb_hid::report::field::UsbHidReportField;
use hid_types::id::CollectionType;
use hid_types::item::usage::ExtendedUsage;
use std::iter;
use std::sync::Arc;

pub mod builder;
pub mod error;
pub mod field;

#[derive(Debug)]
pub struct UsbHidReportDescriptor {
    root_collections: Vec<UsbHidReportCollection>,
}

#[derive(Debug, Clone)]
pub struct UsbHidReport(Arc<UsbHidReportInner>);

#[derive(Debug)]
pub struct UsbHidReportInner {
    report_id: Option<u8>,
    size_in_bits: usize,
    fields: Vec<UsbHidReportField>,
}

#[derive(Debug)]
pub struct UsbHidReportCollection {
    usage: ExtendedUsage,
    collection_type: CollectionType,
    input_reports: Vec<UsbHidReport>,
    output_reports: Vec<UsbHidReport>,
    feature_reports: Vec<UsbHidReport>,
    child_collections: Vec<UsbHidReportCollection>,
}

impl UsbHidReportDescriptor {
    pub fn all_input_reports(&self) -> impl Iterator<Item = &UsbHidReport> {
        self.root_collections
            .iter()
            .flat_map(UsbHidReportCollection::all_input_reports)
    }

    pub fn all_output_reports(&self) -> impl Iterator<Item = &UsbHidReport> {
        self.root_collections
            .iter()
            .flat_map(UsbHidReportCollection::all_output_reports)
    }

    pub fn all_feature_reports(&self) -> impl Iterator<Item = &UsbHidReport> {
        self.root_collections
            .iter()
            .flat_map(UsbHidReportCollection::all_feature_reports)
    }

    pub fn root_collections(&self) -> impl Iterator<Item = &UsbHidReportCollection> {
        self.root_collections.iter()
    }

    pub fn all_collections(&self) -> impl Iterator<Item = &UsbHidReportCollection> {
        self.root_collections.iter().flat_map(|root_collection| {
            iter::once(root_collection).chain(root_collection.all_child_collections())
        })
    }
}

impl UsbHidReport {
    pub fn new(report_id: Option<u8>, fields: Vec<UsbHidReportField>) -> Self {
        let size_in_bits = 8 + fields
            .iter()
            .map(UsbHidReportField::size_in_bits)
            .sum::<usize>();

        let inner = UsbHidReportInner {
            report_id,
            size_in_bits,
            fields,
        };

        Self(Arc::new(inner))
    }

    pub fn report_id(&self) -> Option<u8> {
        self.0.report_id
    }

    pub fn size_in_bits(&self) -> usize {
        self.0.size_in_bits
    }

    pub fn size_in_bytes(&self) -> usize {
        self.size_in_bits().div_ceil(8)
    }

    pub fn fields(&self) -> impl Iterator<Item = &UsbHidReportField> {
        self.0.fields.iter()
    }
}

impl UsbHidReportCollection {
    pub fn new(
        usage: ExtendedUsage,
        collection_type: CollectionType,
        input_reports: Vec<UsbHidReport>,
        output_reports: Vec<UsbHidReport>,
        feature_reports: Vec<UsbHidReport>,
        child_collections: Vec<UsbHidReportCollection>,
    ) -> Self {
        Self {
            usage,
            collection_type,
            input_reports,
            output_reports,
            feature_reports,
            child_collections,
        }
    }

    pub fn usage(&self) -> ExtendedUsage {
        self.usage
    }

    pub fn collection_type(&self) -> CollectionType {
        self.collection_type
    }

    pub fn all_input_reports(&self) -> impl Iterator<Item = &UsbHidReport> {
        self.input_reports.iter().chain(
            self.all_child_collections()
                .flat_map(|child_collection| child_collection.input_reports()),
        )
    }

    pub fn input_reports(&self) -> impl Iterator<Item = &UsbHidReport> {
        self.input_reports.iter()
    }

    pub fn all_output_reports(&self) -> impl Iterator<Item = &UsbHidReport> {
        self.output_reports.iter().chain(
            self.all_child_collections()
                .flat_map(|child_collection| child_collection.output_reports()),
        )
    }

    pub fn output_reports(&self) -> impl Iterator<Item = &UsbHidReport> {
        self.output_reports.iter()
    }

    pub fn all_feature_reports(&self) -> impl Iterator<Item = &UsbHidReport> {
        self.feature_reports.iter().chain(
            self.all_child_collections()
                .flat_map(|child_collection| child_collection.feature_reports()),
        )
    }

    pub fn feature_reports(&self) -> impl Iterator<Item = &UsbHidReport> {
        self.feature_reports.iter()
    }

    pub fn all_child_collections(&self) -> Box<dyn Iterator<Item = &UsbHidReportCollection> + '_> {
        if self.child_collections.is_empty() {
            Box::new(iter::empty())
        } else {
            Box::new(self.child_collections.iter().flat_map(|child_collection| {
                iter::once(child_collection).chain(child_collection.all_child_collections())
            }))
        }
    }

    pub fn child_collections(&self) -> impl Iterator<Item = &UsbHidReportCollection> {
        self.child_collections.iter()
    }
}
