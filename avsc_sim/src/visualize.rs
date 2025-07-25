use crate::model::{Module, GateType};

pub fn print_gate_level(module: &Module) {
    println!("\nGate-Level Schematic:");
    for (i, gate) in module.gates.iter().enumerate() {
        let in_names: Vec<_> = gate.inputs.iter().map(|&sid| &module.signals[sid].name).collect();
        let out_name = &module.signals[gate.output].name;
        println!("[{}] {:?}: {} -> {}", i, gate.gate_type, in_names.join(", "), out_name);
    }
    println!("\nSignal Connections:");
    for (sid, sig) in module.signals.iter().enumerate() {
        let mut conn = format!("{}", sig.name);
        if !sig.drivers.is_empty() {
            conn = format!("{} <-- {}", conn, sig.drivers.iter().map(|&gid| format!"[{}]", gid).collect::<Vec<_>>().join(", "));
        }
        if !sig.loads.is_empty() {
            conn = format!("{} --> {}", conn, sig.loads.iter().map(|&gid| format!"[{}]", gid).collect::<Vec<_>>().join(", "));
        }
        println!("{}", conn);
    }
}