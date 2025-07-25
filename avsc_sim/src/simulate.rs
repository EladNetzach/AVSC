use crate::model::{GateType, Module, SignalId};
use std::collections::{HashSet, VecDeque};

/// Simulate the module for a given input vector, event-driven (combinational only)
pub fn simulate_module(module: &Module, input_values: &[bool]) -> Vec<bool> {
    let mut signal_values = vec![false; module.signals.len()];
    // Set input values
    for (i, &sig_id) in module.input_ids.iter().enumerate() {
        signal_values[sig_id] = input_values[i];
    }
    // Event-driven simulation: queue of gates to process
    let mut queue: VecDeque<_> = (0..module.gates.len()).collect();
    let mut processed = HashSet::new();
    while let Some(gate_id) = queue.pop_front() {
        if !processed.insert(gate_id) {
            continue;
        }
        let gate = &module.gates[gate_id];
        let input_vals: Vec<bool> = gate.inputs.iter().map(|&sid| signal_values[sid]).collect();
        let out_val = match gate.gate_type {
            GateType::And => input_vals.iter().all(|&v| v),
            GateType::Or => input_vals.iter().any(|&v| v),
            GateType::Not => !input_vals[0],
            GateType::Nand => !input_vals.iter().all(|&v| v),
            GateType::Nor => !input_vals.iter().any(|&v| v),
            GateType::Xor => input_vals.iter().fold(false, |acc, &v| acc ^ v),
            GateType::Buf => input_vals[0],
        };
        // If output changes, enqueue all gates that use this output
        let out_id = gate.output;
        if signal_values[out_id] != out_val {
            signal_values[out_id] = out_val;
            for &load_gate in &module.signals[out_id].loads {
                queue.push_back(load_gate);
            }
        }
    }
    // Collect output values
    module.output_ids.iter().map(|&oid| signal_values[oid]).collect()
}