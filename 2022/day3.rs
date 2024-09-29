fn increment_storage(idx: usize, first: &mut [i32; 52], second: &mut [i32; 52], first_half: bool) {
    if first_half {
        first[idx] += 1;
    } else {
        second[idx] += 1;
    }
}

fn part1(content: String) {
    let mut ans = 0;

    for line in content.lines() {
        let mut first = [0 as i32; 52];
        let mut second = [0 as i32; 52];
        let first_half = line.len() / 2;
        let mut count = 0;

        for c in line.bytes() {
            count += 1;
            let mut idx = (c - b'A' + 26) as usize;
            if c >= b'a' && c <= b'z' {
                idx = (c - b'a') as usize;
            }
            increment_storage(idx, &mut first, &mut second, count <= first_half);
        }

        for i in 0..52 {
            if first[i] != 0 && second[i] != 0 {
                ans += i + 1;
            }
        }
    }
    println!("part1: {}", ans);
}

fn part2(content: String) {
    let mut ans = 0;
    let mut count = 0;
    let mut bag = [[0 as i32; 52], [0 as i32; 52], [0 as i32; 52]];

    for line in content.lines() {
        for c in line.bytes() {
            let mut idx = (c - b'A' + 26) as usize;
            if c >= b'a' && c <= b'z' {
                idx = (c - b'a') as usize;
            }
            bag[count][idx] += 1;
        }
        count += 1;
        if count == 3 {
            count = 0;
            for i in 0..52 {
                if bag[0][i] != 0 && bag[1][i] != 0 && bag[2][i] != 0 {
                    ans += i + 1;
                }
            }
            bag = [[0 as i32; 52], [0 as i32; 52], [0 as i32; 52]];
        }
    }
    println!("part2: {}", ans);
}

fn main() {
    let content = std::fs::read_to_string("input").expect("File 'input' not found!");
    part1(content.clone());
    part2(content.clone());
}
