
#[derive(Debug, Clone, Default, PartialEq)]
/// A struct containing the information from the .RAW header
pub struct HeaderInfo {
    /// ## IC: New Case Flag
    /// ### Type: i8
    /// ### Default: 0
    /// ## Parameters
    /// * 0 : Base Case Input
    /// * 1 : Add new data to working case
    pub ic: i8,
    /// ## sbase: System MVA Base
    /// ### Type: f64
    /// ### Default: 100.0
    pub sbase: f64,
    /// ## revision: PSS/E revision number
    /// ### Type: i8
    /// ### Default: 34
    pub revision: i8,
    /// ## XFRRAT: Units of the transformer ratings
    /// ### Type: i8
    /// ### Default: 0
    /// ## Parameters
    /// * XFRRAT <= 0 : MVA
    /// * XFRRAT > 0 : Current expressed as MVA
    pub transformer_rating_code: i8,
    /// ## NXFRAT: Units of the non-transformer branch ratings
    /// ### Type: i8
    /// ### Default: 0
    /// ## Parameters
    /// * NXFRAT <= 0 : MVA
    /// * NXFRAT > 0 : Current expressed as MVA
    pub branch_rating_code: i8,
    /// ## BASFRQ: System base frequency in Hertz (Hz)
    /// ### Type: f64
    /// ### Default: 60.0
    pub system_frequency: f64,
}

impl From<Vec<String>> for HeaderInfo {

    fn from(values: Vec<String>) -> Self {

        HeaderInfo {
            ic: values.get(0).and_then(|s| s.parse().ok()).unwrap_or(0),
            sbase: values.get(1).and_then(|s| s.parse().ok()).unwrap_or(100.0),
            revision: values.get(2).and_then(|s| s.parse().ok()).unwrap_or(34),
            transformer_rating_code: values.get(3).and_then(|s| s.parse().ok()).unwrap_or(0),
            branch_rating_code: values.get(4).and_then(|s| s.parse().ok()).unwrap_or(0),
            system_frequency: values.get(5).and_then(|s| s.parse().ok()).unwrap_or(60.0),
        }
    }
}