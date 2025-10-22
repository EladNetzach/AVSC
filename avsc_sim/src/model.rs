use std::collections::HashMap;

pub type SignalId = usize;
pub type GateId = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateType {
    And,
    Or,
    Not,
    Nand,
    Nor,
    Xor,
    Buf,
}

#[derive(Debug, Clone)]
pub struct Signal {
    pub name: String,
    pub value: Option<bool>,
    pub drivers: Vec<GateId>, // Gates that drive this signal
    pub loads: Vec<GateId>,   // Gates that use this signal as input
}

#[derive(Debug, Clone)]
pub struct Gate {
    pub name: String,
    pub gate_type: GateType,
    pub inputs: Vec<SignalId>,
    pub output: SignalId,
}

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub input_ids: Vec<SignalId>,
    pub output_ids: Vec<SignalId>,
    pub gates: Vec<Gate>,
    pub signals: Vec<Signal>,
}

#[derive(Debug, Clone)]
pub struct Circuit {
    pub top: Module,
    pub signals: HashMap<String, Signal>,
}