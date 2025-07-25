# AVSC Sim

A Virtual Silicon Chip Simulation Platform: Parse Verilog, simulate, and visualize all abstraction layers (starting with CLI simulation).

## Features
- Parse simple Verilog files (modules, inputs, outputs, wires, assign statements)
- Simulate combinational logic (AND, OR, NOT, XOR, BUF)
- CLI interface for entering input values and viewing outputs
- Modular code for future expansion (RTL, gate, transistor, layout, GDSII, etc.)

## Usage

1. **Build the project:**
   ```bash
   cargo build --release
   ```
2. **Create a Verilog file (e.g., `example.v`):**
   ```verilog
   module simple_and(input A, input B, output Y);
       assign Y = A & B;
   endmodule
   ```
3. **Run the simulation:**
   ```bash
   cargo run --release -- example.v
   ```
   Enter values for each input when prompted.

## Example Output
```
Parsed module: simple_and
Inputs: ["A", "B"]
Outputs: ["Y"]
Gates: [Gate { name: "Y_gate", gate_type: And, inputs: ["A", "B"], output: "Y" }]
Enter value for A (0 or 1): 1
Enter value for B (0 or 1): 0

Simulation Results:
  Y = 0
```

## Roadmap
- [x] Verilog parser and CLI simulation
- [ ] RTL, gate, and transistor-level visualization
- [ ] GDSII parsing and physical layout visualization
- [ ] GUI (Bevy) for interactive navigation

## License
MIT