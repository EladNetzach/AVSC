mod model;
mod parser;
mod simulate;
mod visualize;

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
    let input_names: Vec<_> = module.input_ids.iter().map(|&id| &module.signals[id].name).collect();
    let output_names: Vec<_> = module.output_ids.iter().map(|&id| &module.signals[id].name).collect();
    println!("Inputs: {:?}", input_names);
    println!("Outputs: {:?}", output_names);
    println!("Gates: {:?}", module.gates);
    visualize::print_gate_level(&module);

    // Generate all input combinations
    let n = module.input_ids.len();
    let num_combinations = 1 << n;
    println!("\nTruth Table:");
    // Print header
    for name in &input_names {
        print!("{} ", name);
    }
    print!("| ");
    for name in &output_names {
        print!("{} ", name);
    }
    println!();
    // Simulate for each combination
    for i in 0..num_combinations {
        let mut input_vec = vec![false; n];
        for (j, _) in module.input_ids.iter().enumerate() {
            input_vec[j] = (i >> (n - 1 - j)) & 1 == 1;
            print!("{} ", input_vec[j] as u8);
        }
        print!("| ");
        let outputs = simulate_module(&module, &input_vec);
        for &val in &outputs {
            print!("{} ", val as u8);
        }
        println!();
    }
}