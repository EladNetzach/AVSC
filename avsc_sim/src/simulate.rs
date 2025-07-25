use crate::model::{Circuit, GateType, Module, Signal};
use std::collections::HashMap;

pub fn simulate_module(module: &Module, input_values: &HashMap<String, bool>) -> HashMap<String, bool> {
    let mut signals: HashMap<String, bool> = HashMap::new();
    // Set input values
    for input in &module.inputs {
        if let Some(&val) = input_values.get(input) {
            signals.insert(input.clone(), val);
        }
    }
    // Simulate gates (single pass, combinational only)
    for gate in &module.gates {
        let input_vals: Vec<bool> = gate.inputs.iter().map(|name| *signals.get(name).unwrap_or(&false)).collect();
        let out_val = match gate.gate_type {
            GateType::And => input_vals.iter().all(|&v| v),
            GateType::Or => input_vals.iter().any(|&v| v),
            GateType::Not => !input_vals[0],
            GateType::Nand => !input_vals.iter().all(|&v| v),
            GateType::Nor => !input_vals.iter().any(|&v| v),
            GateType::Xor => input_vals.iter().fold(false, |acc, &v| acc ^ v),
            GateType::Buf => input_vals[0],
        };
        signals.insert(gate.output.clone(), out_val);
    }
    // Collect output values
    let mut outputs = HashMap::new();
    for output in &module.outputs {
        outputs.insert(output.clone(), *signals.get(output).unwrap_or(&false));
    }
    outputs
}