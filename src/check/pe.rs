//! Defines the `PE` security mitigation checker. Consumes an
//! PE binary, parses it, and checks for the following features:
//!
//! * Authenticode
//! * Data Execution PRevention
//! * ASLR
//! * Dynamic Base

use goblin::pe::PE;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::check::{BinFeatures, Checker, FeatureMap};

use std::boxed::Box;

/// struct defining parsed info given a PE binary format
#[derive(Deserialize, Serialize, Default)]
pub struct PeInfo {
    pub machine: u16,
    pub num_sections: u16,
    pub timestamp: u32,
}

#[typetag::serde]
impl BinFeatures for PeInfo {
    fn dump_mapping(&self) -> FeatureMap {
        let mut features: FeatureMap = FeatureMap::new();
        features.insert("Machine", json!(self.machine));
        features.insert("Number of Sections", json!(self.num_sections));
        features.insert("Timestamp", json!(self.timestamp));
        features
    }
}

/// struct defining security features parsed from PE, and
/// derives serde de/serialize traits for structured output.
#[derive(Deserialize, Serialize)]
pub struct PeChecker {}

#[typetag::serde]
impl BinFeatures for PeChecker {
    fn dump_mapping(&self) -> FeatureMap {
        let features: FeatureMap = FeatureMap::new();
        features
    }
}

impl Checker for PE<'_> {
    /// parses out basic binary information and stores for consumption and output.
    fn bin_info(&self) -> Box<dyn BinFeatures> {
        Box::new(PeInfo {
            machine: self.header.coff_header.machine,
            num_sections: self.header.coff_header.number_of_sections,
            timestamp: self.header.coff_header.time_date_stamp,
        })
    }

    /// implements the necesary checks for the security mitigations for the specific file format.
    fn harden_check(&self) -> Box<dyn BinFeatures> {
        Box::new(PeChecker {})
    }
}