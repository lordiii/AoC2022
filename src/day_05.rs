struct Instruction {
    source: u8,
    target: u8,
    amount: u8,
}

pub fn main(puzzle_input: &str) {
    let parts: Vec<&str> = puzzle_input.split("\n\n").collect();

    if parts.len() == 2 {
        let stack: Vec<&Vec<u8>> = extract_stacks(parts[0]);
        let instructions = extract_instructions(parts[1]);

        println!("Instructions: \n{}", instruction_string);
    } else {
        panic!("Encountered invalid input string!");
    }
}

fn extract_stacks(stack_string: &str) -> [&Vec<u8>] {
    let chars: Vec<&str> = stack_string.split("\n").collect();
    let mut i: u8 = 0;

    for chr in chars {
        println!("{}: {}", i, chr);

        i += 1;
    }
}

fn extract_instructions(instruction_string: &str) -> Vec<&Instruction> {
    let instructions: Vec<&str> = instruction_string.split("\n").collect();
    let mut result: Vec<&Instruction> = Vec::with_capacity(instructions.len());
    for instruction in instructions {}

    return result;
}