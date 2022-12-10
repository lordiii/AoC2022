use regex::Regex;

struct Instruction {
    source: usize,
    target: usize,
    amount: usize,
}

pub fn main(puzzle_input: &str) {
    let parts: Vec<&str> = puzzle_input.split("\n\n").collect();

    if parts.len() == 2 {
        let mut stacks: Vec<Vec<String>> = extract_stacks(parts[0]);
        let mut m_stacks: Vec<Vec<String>> = stacks.clone();
        let instructions: Vec<Instruction> = extract_instructions(parts[1]);

        for instr in instructions {
            let mut v: Vec<String> = Vec::with_capacity(instr.amount);

            for _ in 0..instr.amount {
                let s_pop = stacks[instr.source].pop().unwrap();
                stacks[instr.target].push(s_pop);

                let m_pop = m_stacks[instr.source].pop().unwrap();
                v.push(m_pop);
            }


            v.reverse();
            for str in v {
                m_stacks[instr.target].push(str);
            }
        }

        print!("Top of Stacks: ");
        for i in stacks {
            print!("{}", i.last().unwrap());
        }

        println!();

        print!("Top of Stacks when multiple are taken: ");
        for i in m_stacks {
            print!("{}", i.last().unwrap());
        }

        println!();
    } else {
        panic!("Encountered invalid input string!");
    }
}

fn extract_stacks(stack_string: &str) -> Vec<Vec<String>> {
    let re = Regex::new(r"( {4})|(\[\w])").unwrap();
    let count_re = Regex::new(r"(\d+)").unwrap();

    let stack_size: u8 = count_re.captures_iter(stack_string).last().unwrap()[0].parse::<u8>().unwrap();
    println!("Amount of stacks: {}", stack_size);

    let mut vec: Vec<Vec<String>> = vec![Vec::new(); stack_size as usize];

    let mut i: usize = 0;
    for capture in re.captures_iter(stack_string) {
        let s: String = capture[0].to_owned().replace("[", "").replace("]", "");

        if !s.trim().is_empty() {
            vec[(i % stack_size as usize) as usize].insert(0, s);
        }

        i += 1;
    }

    return vec;
}

fn extract_instructions(instruction_string: &str) -> Vec<Instruction> {
    let count_re: Regex = Regex::new(r"(\d+)").unwrap();
    let mut result: Vec<Instruction> = Vec::new();

    let mut i: usize = 0;
    for capture in count_re.captures_iter(instruction_string) {
        let s = capture[0].to_owned();

        if i % 3 == 0 {
            result.push(Instruction {
                amount: 0,
                source: 0,
                target: 0,
            });
        }

        let number: usize = s.parse::<usize>().unwrap();

        match i % 3 {
            0 => result[i / 3].amount = number,
            1 => result[i / 3].source = number - 1,
            2 => result[i / 3].target = number - 1,
            _ => ()
        };

        i += 1;
    }

    return result;
}