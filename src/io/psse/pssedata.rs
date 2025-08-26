use super::components::structs::*;

// Enum to keep track of which section we are parsing
#[derive(Debug, PartialEq, Eq)]
pub enum PSSEDatatype {
    Header,
    Bus,
    Load,
    FixedShunt,
    Generator,
    Branch,
    Transformer,
    Area,
    TwoTerminalDc,
    VscDc,
    ImpedanceCorrection,
    MultiTerminalDc,
    MultiSectionLine,
    Zone,
    InterAreaTransfer,
    Owner,
    Facts,
    SwitchedShunt,
    GneDevice,
    Unknown, // For lines we don't recognize or sections not yet handled
    EndOfFile,
}

#[derive(Debug, Clone, Default)]
pub struct PSSEData {
    pub header: HeaderInfo,
    pub buses: Vec<Bus>,
    pub loads: Vec<Load>,
    pub fixed_shunts: Vec<FixedShunt>,
    pub generators: Vec<Generator>,
    pub branches: Vec<Branch>,
    pub switching_devices: Vec<SystemSwitchingDevice>,
    pub transformers: Vec<Transformer>,
    pub areas: Vec<Area>,
    pub two_terminal_dc: Vec<TwoTerminalDc>,
    pub vsc_dc: Vec<VSCDc>,
    pub impedance_correction: Vec<ImpedanceCorrectionTable>,
    pub multi_terminal_line: Vec<MultiTermDCLine>,
    pub multi_section_line: Vec<MultiSectionLine>,
    pub zones: Vec<Zone>,
    pub inter_area_transfer: Vec<InterAreaTransfer>,
    pub owners: Vec<Owner>,
    pub facts: Vec<Facts>,
    pub switched_shunts: Vec<SwitchedShunt>,
    pub induction_machines: Vec<InductionMachine>,
}