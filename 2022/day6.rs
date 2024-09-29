fn solution(content: String, marker_size: usize) {
    let mut ans = 0;
    let mut letters = [0; 26];
    let content = content.chars().collect::<Vec<char>>();
    let mut p = 0;
    letters[(content[p] as u8 - b'a') as usize] = 1;

    for i in 1..content.len() {
        while letters[(content[i] as u8 - b'a') as usize] != 0 {
            letters[(content[p] as u8 - b'a') as usize] -= 1;
            p += 1;
        }
        letters[(content[i] as u8 - b'a') as usize] += 1;
        if i - p == marker_size {
            ans = i + 1;
            break;
        }
    }
    println!("{}", ans);
}

fn main() {
    let content = std::fs::read_to_string("input").expect("File 'input' not found!");
    solution(content.clone(), 3);
    solution(content.clone(), 13);
}
