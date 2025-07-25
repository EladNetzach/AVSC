mod model;
mod parser;
mod simulate;

use std::collections::HashMap;
use std::env;
use std::fs;

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

    // Generate all input combinations
    let n = module.inputs.len();
    let num_combinations = 1 << n;
    println!("\nTruth Table:");
    // Print header
    for input in &module.inputs {
        print!("{} ", input);
    }
    print!("| ");
    for output in &module.outputs {
        print!("{} ", output);
    }
    println!();
    // Simulate for each combination
    for i in 0..num_combinations {
        let mut input_values = HashMap::new();
        for (j, input) in module.inputs.iter().enumerate() {
            let val = (i >> (n - 1 - j)) & 1 == 1;
            input_values.insert(input.clone(), val);
            print!("{} ", val as u8);
        }
        print!("| ");
        let outputs = simulate_module(&module, &input_values);
        for output in &module.outputs {
            print!("{} ", outputs.get(output).copied().unwrap_or(false) as u8);
        }
        println!();
    }
}