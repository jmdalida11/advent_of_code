fn get_move_value(your_move: &str) -> i32 {
    if your_move == "X" {
        return 1;
    } else if your_move == "Y" {
        return 2;
    }
    return 3;
}

fn you_win(your_move: &str, opponent_move: &str) -> bool {
    return (your_move == "Y" && opponent_move == "A")
        || (your_move == "X" && opponent_move == "C")
        || (your_move == "Z" && opponent_move == "B");
}

fn is_equal(your_move: &str, opponent_move: &str) -> bool {
    return (your_move == "X" && opponent_move == "A")
        || (your_move == "Y" && opponent_move == "B")
        || (your_move == "Z" && opponent_move == "C");
}

fn part1(content: String) {
    let mut score = 0;
    for line in content.lines() {
        let text: Vec<&str> = line.split_whitespace().collect();
        let opponent_move = text[0];
        let your_move = text[1];

        score += get_move_value(your_move);
        if is_equal(your_move, opponent_move) {
            score += 3;
        } else if you_win(your_move, opponent_move) {
            score += 6;
        }
    }
    println!("{}", score);
}

fn move_value(opponent_move: &str, should_win: bool) -> i32 {
    if opponent_move == "A" {
        return if should_win { 2 } else { 3 };
    } else if opponent_move == "B" {
        return if should_win { 3 } else { 1 };
    }
    return if should_win { 1 } else { 2 };
}

fn draw_move_value(opponent_move: &str) -> i32 {
    if opponent_move == "A" {
        return 1;
    } else if opponent_move == "B" {
        return 2;
    }
    return 3;
}

fn part2(content: String) {
    let mut score = 0;
    for line in content.lines() {
        let text: Vec<&str> = line.split_whitespace().collect();
        let opponent_move = text[0];
        let your_move = text[1];

        match your_move {
            "Z" => {
                score += 6;
                score += move_value(opponent_move, true);
            }
            "Y" => {
                score += 3;
                score += draw_move_value(opponent_move);
            }
            "X" => {
                score += move_value(opponent_move, false);
            }
            _ => {}
        }
    }
    println!("{}", score);
}

fn main() {
    let content = std::fs::read_to_string("input").expect("File 'input' not found!");
    part1(content.clone());
    part2(content.clone());
}
