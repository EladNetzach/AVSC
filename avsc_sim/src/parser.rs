use crate::model::{Gate, GateType, Module, Signal, SignalId, GateId};
use regex::Regex;
use std::collections::HashMap;

pub fn parse_verilog(source: &str) -> Module {
    let mut name = String::new();
    let mut signal_ids: HashMap<String, SignalId> = HashMap::new();
    let mut signals: Vec<Signal> = Vec::new();
    let mut gates: Vec<Gate> = Vec::new();
    let mut input_ids = Vec::new();
    let mut output_ids = Vec::new();

    let re_module = Regex::new(r"module\s+(\w+)\s*\(([^)]*)\)").unwrap();
    let re_input = Regex::new(r"input\s+([\w, ]+);?").unwrap();
    let re_output = Regex::new(r"output\s+([\w, ]+);?").unwrap();
    let re_wire = Regex::new(r"wire\s+([\w, ]+);?").unwrap();
    let re_assign = Regex::new(r"assign\s+(\w+)\s*=\s*([\w~&|^() ]+);?").unwrap();

    // Helper to get or create a signal id
    let mut get_signal_id = |name: &str| -> SignalId {
        if let Some(&id) = signal_ids.get(name) {
            id
        } else {
            let id = signals.len();
            signals.push(Signal {
                name: name.to_string(),
                value: None,
                drivers: Vec::new(),
                loads: Vec::new(),
            });
            signal_ids.insert(name.to_string(), id);
            id
        }
    };

    for line in source.lines() {
        let line = line.trim();
        if let Some(cap) = re_module.captures(line) {
            name = cap[1].to_string();
        } else if let Some(cap) = re_input.captures(line) {
            for s in cap[1].split(',').map(|s| s.trim()) {
                let id = get_signal_id(s);
                input_ids.push(id);
            }
        } else if let Some(cap) = re_output.captures(line) {
            for s in cap[1].split(',').map(|s| s.trim()) {
                let id = get_signal_id(s);
                output_ids.push(id);
            }
        } else if let Some(cap) = re_wire.captures(line) {
            for s in cap[1].split(',').map(|s| s.trim()) {
                get_signal_id(s);
            }
        } else if let Some(cap) = re_assign.captures(line) {
            let out = cap[1].trim();
            let expr = cap[2].trim();
            let (gate_type, input_names): (GateType, Vec<&str>) = if expr.contains('&') {
                (GateType::And, expr.split('&').map(|s| s.trim()).collect())
            } else if expr.contains('|') {
                (GateType::Or, expr.split('|').map(|s| s.trim()).collect())
            } else if expr.starts_with('~') {
                (GateType::Not, vec![expr[1..].trim()])
            } else if expr.contains('^') {
                (GateType::Xor, expr.split('^').map(|s| s.trim()).collect())
            } else {
                (GateType::Buf, vec![expr])
            };
            let input_ids: Vec<SignalId> = input_names.iter().map(|&n| get_signal_id(n)).collect();
            let output_id = get_signal_id(out);
            let gate_id = gates.len();
            for &in_id in &input_ids {
                signals[in_id].loads.push(gate_id);
            }
            signals[output_id].drivers.push(gate_id);
            gates.push(Gate {
                name: format!("{}_gate", out),
                gate_type,
                inputs: input_ids,
                output: output_id,
            });
        }
    }

    Module {
        name,
        input_ids,
        output_ids,
        gates,
        signals,
    }
}