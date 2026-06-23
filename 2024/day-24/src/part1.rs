use crate::{execute, parse_input};
use common::custom_error::{Error, Result};
use std::collections::VecDeque;

pub fn process(input: &str) -> Result<String, Error> {
    let (_, (mut states, gates)) = parse_input(input).expect("Should parse");

    // parse input
    // println!("States: {:?}", states);
    // println!("Gates: {:?}", gates);

    // every gate has two inputs and one output
    // make a stack of work
    // multiple gates can share the same input/output except for the final zxx lines.
    // let gates: Vec<Gate> = Vec::new();
    // let mut states: HashMap<&str, bool> = HashMap::new(); // only start with input set

    // add all the gates
    let mut queue = VecDeque::new();
    for gate in gates.iter() {
        queue.push_back(gate);
    }

    while let Some(gate) = queue.pop_front() {
        if states.contains_key(&gate.input1) && states.contains_key(&gate.input2) {
            let a = states[&gate.input1];
            let b = states[&gate.input2];
            let output = execute(a, b, gate.gate_type);
            *states.entry(gate.output).or_insert(output) = output;
        } else {
            queue.push_back(gate);
        }
    }

    // collect the zxx outputs and convert to a number
    let result = states
        .iter()
        .filter(|(&k, _)| k.starts_with("z"))
        .map(|(&k, &v)| (k.strip_prefix("z").unwrap().parse::<u64>().unwrap(), v))
        .fold(0u64, |mut acc, (k, v)| {
            let bit = v as u64;
            let mult = 1 << k;
            acc += bit * mult;

            acc
        });

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
        assert_eq!("4", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_larger() -> Result<()> {
        let input = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
