use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Signal {
    pub name: String,
    pub value: Option<bool>, // None = unknown, Some(true/false) = logic value
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateType {
    And,
    Or,
    Not,
    Nand,
    Nor,
    Xor,
    Buf,
    // Add more as needed
}

#[derive(Debug, Clone)]
pub struct Gate {
    pub name: String,
    pub gate_type: GateType,
    pub inputs: Vec<String>, // Signal names
    pub output: String,      // Signal name
}

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub wires: Vec<String>,
    pub gates: Vec<Gate>,
}

#[derive(Debug, Clone)]
pub struct Circuit {
    pub top: Module,
    pub signals: HashMap<String, Signal>,
}