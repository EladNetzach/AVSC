mod model;
mod parser;
mod simulate;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};

use model::Module;
use parser::parse_verilog;
use simulate::simulate_module;

fn main() {
    // Get Verilog file from command line
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <verilog_file>", args[0]);
        return;
    }
    let filename = &args[1];
    let source = fs::read_to_string(filename).expect("Failed to read file");
    let module: Module = parse_verilog(&source);
    println!("Parsed module: {}", module.name);
    println!("Inputs: {:?}", module.inputs);
    println!("Outputs: {:?}", module.outputs);
    println!("Gates: {:?}", module.gates);

    // Prompt for input values
    let mut input_values = HashMap::new();
    for input in &module.inputs {
        print!("Enter value for {} (0 or 1): ", input);
        io::stdout().flush().unwrap();
        let mut val = String::new();
        io::stdin().read_line(&mut val).unwrap();
        let v = val.trim() == "1";
        input_values.insert(input.clone(), v);
    }

    // Simulate
    let outputs = simulate_module(&module, &input_values);
    println!("\nSimulation Results:");
    for (out, val) in outputs {
        println!("  {} = {}", out, val as u8);
    }
}