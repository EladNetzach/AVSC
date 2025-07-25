use crate::model::{Gate, GateType, Module};
use regex::Regex;

pub fn parse_verilog(source: &str) -> Module {
    let mut name = String::new();
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    let mut wires = Vec::new();
    let mut gates = Vec::new();

    let re_module = Regex::new(r"module\s+(\w+)\s*\(([^)]*)\)").unwrap();
    let re_input = Regex::new(r"input\s+([\w, ]+);?").unwrap();
    let re_output = Regex::new(r"output\s+([\w, ]+);?").unwrap();
    let re_wire = Regex::new(r"wire\s+([\w, ]+);?").unwrap();
    let re_assign = Regex::new(r"assign\s+(\w+)\s*=\s*([\w~&|^() ]+);?").unwrap();

    for line in source.lines() {
        let line = line.trim();
        if let Some(cap) = re_module.captures(line) {
            name = cap[1].to_string();
        } else if let Some(cap) = re_input.captures(line) {
            inputs.extend(cap[1].split(',').map(|s| s.trim().to_string()));
        } else if let Some(cap) = re_output.captures(line) {
            outputs.extend(cap[1].split(',').map(|s| s.trim().to_string()));
        } else if let Some(cap) = re_wire.captures(line) {
            wires.extend(cap[1].split(',').map(|s| s.trim().to_string()));
        } else if let Some(cap) = re_assign.captures(line) {
            let out = cap[1].trim().to_string();
            let expr = cap[2].trim();
            // Only support simple assign: out = in1 & in2, out = in1 | in2, out = ~in1, etc.
            let (gate_type, inputs): (GateType, Vec<String>) = if expr.contains('&') {
                (GateType::And, expr.split('&').map(|s| s.trim().to_string()).collect())
            } else if expr.contains('|') {
                (GateType::Or, expr.split('|').map(|s| s.trim().to_string()).collect())
            } else if expr.starts_with('~') {
                (GateType::Not, vec![expr[1..].trim().to_string()])
            } else if expr.contains('^') {
                (GateType::Xor, expr.split('^').map(|s| s.trim().to_string()).collect())
            } else {
                // Buffer or direct assignment
                (GateType::Buf, vec![expr.to_string()])
            };
            gates.push(Gate {
                name: format!("{}_gate", out),
                gate_type,
                inputs,
                output: out,
            });
        }
    }

    Module {
        name,
        inputs,
        outputs,
        wires,
        gates,
    }
}