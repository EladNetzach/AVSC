/// Abstraction layers in the AVSC simulation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer {
    System,
    RTL,
    ModuleInternals,
    LogicGates,
    NetConnections,
    TransistorLogic,
    StandardCellLayout,
    MaskLevelGeometry,
}

impl Layer {
    pub fn name(&self) -> &'static str {
        match self {
            Layer::System => "System Level",
            Layer::RTL => "RTL Functional Blocks",
            Layer::ModuleInternals => "Module Internals",
            Layer::LogicGates => "Logic Gates Layer",
            Layer::NetConnections => "Net Connections Layer",
            Layer::TransistorLogic => "Transistor Logic Layer",
            Layer::StandardCellLayout => "Standard Cell Layout",
            Layer::MaskLevelGeometry => "Mask-Level Geometry",
        }
    }
}