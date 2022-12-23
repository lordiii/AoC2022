struct State {
    cycle: i32,
    reg: i32,
    signal_strength: i32,
}

pub fn main(puzzle_input: &str) {
    let lines: Vec<&str> = puzzle_input.split("\n").collect();

    let mut state: State = State {
        cycle: -1,
        reg: 1,
        signal_strength: 0,
    };

    clock_cycle(&mut state);

    for line in lines {
        let arguments: Vec<&str> = line.split(" ").collect();

        if arguments.len() > 0 {
            let cmd = arguments[0];

            match cmd {
                "noop" => clock_cycle(&mut state),
                "addx" => add_register(&mut state, arguments),
                _ => ()
            }
        }
    }

    println!("Total Signal Strength: {}", state.signal_strength);
}

fn add_register(mut state: &mut State, arguments: Vec<&str>) {
    if arguments.len() >= 1 {
        clock_cycle(state);

        state.reg = state.reg + arguments[1].parse::<i32>().unwrap();
        clock_cycle(state);
    }
}

fn clock_cycle(state: &mut State) {
    state.cycle += 1;
    let pos = state.cycle % 40;

    if pos == 0 {
        println!();
    }

    if pos + 20 == 0 {
        state.signal_strength += state.cycle * state.reg;
    }

    if  (pos == state.reg - 1) || (pos == state.reg) || (pos == state.reg + 1) {
        print!("#");
    } else {
        print!(".");
    }

}