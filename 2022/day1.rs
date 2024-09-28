fn main() {
    let content = std::fs::read_to_string("input").expect("File 'input' not found!");

    let mut current = 0;
    let mut calories = vec![];
    for line in content.lines() {
        if let Ok(v) = line.parse::<i32>() {
            current += v;
        } else {
            calories.push(current);
            current = 0;
        }
    }
    calories.push(current);
    calories.sort_by(|a, b| b.cmp(&a));
    println!("Part 1: {}", calories[0]);
    println!("Part 2: {}", calories.iter().take(3).sum::<i32>());
}
