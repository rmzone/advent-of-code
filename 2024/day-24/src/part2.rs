use crate::{parse_input, Gate, GateType};
use common::custom_error::{Error, Result};
use std::collections::HashSet;

pub fn process(input: &str) -> Result<String, Error> {
    let (_, (mut states, gates)) = parse_input(input).expect("Should parse");

    // build an adder from bit 00 to 44, remember what the carry-out is from the previous stage
    // if we hit the bad gates then swap, rinse and repeat
    let mut gates_to_swap: HashSet<Gate> = HashSet::new();
    let mut carry = "";

    // grow an adder for each gate added and detect gates with issues?
    // note it takes 5 gates to properly add a value with a carry
    // gate_a: x,y XOR -> a
    // gate_b: x,y AND -> b
    // gate_c: carry,a XOR -> z
    // gate_d: carry,a AND -> d
    // gate_e: b,d OR -> c_out

    let default_gate = Gate {
        gate_type: GateType::None,
        input1: "",
        input2: "",
        output: "",
    };

    for bit in 0..states.len() / 2 {
        // add all the gates x and y for the current bit
        let x_bit = format!("x{:02}", bit);
        let y_bit = format!("y{:02}", bit);

        println!("Testing bit {}", bit);
        println!("c_in: {}", &carry);

        let gate_a = find_gate(&x_bit, &y_bit, GateType::Xor, &gates).unwrap_or(default_gate);
        println!("gate_a: {:?}", &gate_a);
        let gate_b = find_gate(&x_bit, &y_bit, GateType::And, &gates).unwrap_or(default_gate);
        println!("gate_b: {:?}", &gate_b);
        let gate_c = find_gate(carry, gate_a.output, GateType::Xor, &gates).unwrap_or(default_gate);
        println!("gate_c: {:?}", &gate_c);
        let gate_d = find_gate(carry, gate_a.output, GateType::And, &gates).unwrap_or(default_gate);
        println!("gate_d: {:?}", &gate_d);

        let mut swap_gate: Option<&str> = None;

        if bit > 0
            && (gate_a.output.starts_with("z")
                || gate_a.output != gate_c.input1 && gate_a.output != gate_c.input2)
        {
            gates_to_swap.insert(gate_a);
            swap_gate = Some(gate_a.output);
            println!("swapping gate_a: {:?}", gate_a);
        }

        if bit > 0 && !gate_c.output.starts_with("z") {
            gates_to_swap.insert(gate_c);
            if swap_gate.is_none() {
                swap_gate = Some(gate_c.output);
            } else {
                swap_gate = None;
            }
            println!("swapping gate_c: {:?}", gate_c);
        }

        if bit > 0 && gate_b.output.starts_with("z") {
            gates_to_swap.insert(gate_b);
            if swap_gate.is_none() {
                swap_gate = Some(gate_b.output);
            } else {
                swap_gate = None;
            }
            println!("swapping gate_b: {:?}", gate_b);
        }

        if bit > 0 && gate_d.output.starts_with("z") {
            gates_to_swap.insert(gate_d);
            if swap_gate.is_none() {
                swap_gate = Some(gate_d.output);
            } else {
                swap_gate = None;
            }
            println!("swapping gate_d: {:?}", gate_d);
        }

        let e_input = match swap_gate {
            Some(gate) => gate,
            None => {
                if !gate_d.output.starts_with("z") {
                    gate_d.output
                } else if !gate_b.output.starts_with("z") {
                    gate_b.output
                } else {
                    ""
                }
            } // only needs to match one
        };

        let gate_e =
            find_gate(e_input, gate_b.output, GateType::Or, &gates).unwrap_or(default_gate);
        println!("gate_e: {:?}", &gate_e);

        if gate_e.output == "z45" {
            continue; // we are done
        }

        // more checks if the output of the ands
        if bit > 0 && gate_b.output != gate_e.input1 && gate_b.output != gate_e.input2 {
            gates_to_swap.insert(gate_b);
            if swap_gate.is_none() {
                swap_gate = Some(gate_b.output);
            } else {
                swap_gate = None;
            }
            println!("swapping gate_b: {:?}", gate_b);
        }

        if bit > 0 && gate_d.output != gate_e.input1 && gate_d.output != gate_e.input2 {
            gates_to_swap.insert(gate_d);
            if swap_gate.is_none() {
                swap_gate = Some(gate_d.output);
            } else {
                swap_gate = None;
            }
            println!("swapping gate_d: {:?}", gate_d);
        }

        if gate_e.output.starts_with("z") {
            gates_to_swap.insert(gate_e);
            if swap_gate.is_none() {
                panic!("We should of had a gate to swap here!")
            } else {
                carry = swap_gate.unwrap();
            }
            println!("swapping gate_e: {:?}", gate_e);
        } else {
            carry = gate_e.output;
        }

        if bit == 0 {
            carry = gate_b.output;
        }

        println!();
    }

    let mut result: Vec<&str> = gates_to_swap.iter().map(|gate| gate.output).collect();
    result.sort();
    let result = result.join(",");

    Ok(result.to_string())
}

fn find_gate<'a>(a: &str, b: &str, gate_type: GateType, gates: &Vec<Gate<'a>>) -> Option<Gate<'a>> {
    gates
        .iter()
        .find(|&gate| {
            (gate.input2 == a || gate.input2 == b || gate.input1 == a)
                && gate.gate_type == gate_type
        })
        .copied()
}
