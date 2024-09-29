struct Section {
    start: i32,
    end: i32,
}

fn parse_section(range: &str) -> Section {
    if let Some((v1, v2)) = range.split_once('-') {
        return Section {
            start: v1.parse::<i32>().unwrap(),
            end: v2.parse::<i32>().unwrap(),
        };
    }
    panic!("Wrong string format! {}", range);
}

fn fully_overlap(p1: &Section, p2: &Section) -> bool {
    return (p1.start <= p2.start && p1.end >= p2.end)
        || (p1.start >= p2.start && p1.end <= p2.end);
}

fn part1(content: String) {
    let mut ans = 0;
    for line in content.lines() {
        if let Some((first, second)) = line.split_once(',') {
            let p1 = parse_section(first);
            let p2 = parse_section(second);
            if fully_overlap(&p1, &p2) {
                ans += 1;
            }
        } else {
            panic!("Wrong string format! {}", line);
        }
    }
    println!("{}", ans);
}

fn part2(content: String) {
    let mut ans = 0;
    for line in content.lines() {
        if let Some((first, second)) = line.split_once(',') {
            let p1 = parse_section(first);
            let p2 = parse_section(second);
            if (p1.start >= p2.start && p1.start <= p2.end)
                || (p1.end >= p2.start && p1.end <= p2.end)
                || fully_overlap(&p1, &p2)
            {
                ans += 1;
            }
        } else {
            panic!("Wrong string format! {}", line);
        }
    }
    println!("{}", ans);
}

fn main() {
    let content = std::fs::read_to_string("input").expect("File 'input' not found!");
    part1(content.clone());
    part2(content.clone());
}
