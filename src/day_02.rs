pub fn main(puzzle_input: &str) {
    let rounds: Vec<&str> = puzzle_input.split("\n").collect();

    let mut total_strategy_score: u16 = 0;
    let mut total_outcome_score: u16 = 0;

    for round in rounds {
        let players: [u8; 2] = parse_round(round);

        total_strategy_score += as_strategy(players) as u16;
        total_outcome_score += as_outcome(players) as u16;
    }

    println!("As Strategy: {}", total_strategy_score);
    println!("As Outcome: {}", total_outcome_score);
}

fn parse_round(round: &str) -> [u8; 2] {
    let parts: Vec<&str> = round.split_whitespace().collect();

    let mut opponent: u8 = 0;
    let mut player: u8 = 0;

    if parts.len() == 2 {
        opponent = match parts[0] {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => 0
        };

        player = match parts[1] {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => 0
        };
    }

    if player == 0 || opponent == 0 || parts.len() != 2 {
        panic!("Invalid Hand found: {}", round);
    }

    return [opponent, player];
}

fn as_strategy(players: [u8; 2]) -> u8 {
    let opponent: u8 = players[0];
    let player: u8 = players[1];

    if player == opponent {
        return player + 3;
    } else if (player > opponent && !(opponent == 1 && player == 3)) || (player == 1 && opponent == 3) {
        return player + 6;
    }

    return player;
}

fn as_outcome(players: [u8; 2]) -> u8 {
    let opponent: u8 = players[0];
    let player: u8 = players[1];

    return match player {
        1 => match opponent {
            1 => 3,
            2 => 1,
            3 => 2,
            _ => panic!("Invalid opponent state")
        },
        2 => opponent + 3,
        3 => match opponent {
            1 => 2 + 6,
            2 => 3 + 6,
            3 => 1 + 6,
            _ => panic!("Invalid opponent state")
        },
        _ => panic!("Unexpected player state")
    };
}