use std::fs;
use std::cmp::max;

fn part1(trees: &Vec<Vec<i32>>, included: &mut Vec<Vec<bool>>) {
    for r in 0..trees.len() {
        let mut i = 0;
        let mut prev1 = -1;
        let n = trees[0].len() - 1;
        let mut prev2 = -1;
        while i <= n {
            if prev1 < trees[r][i] {
                included[r][i] = true;
            }
            if prev2 < trees[r][n-i] {
                included[r][n-i] = true;
            }
            prev1 = max(prev1, trees[r][i]);
            prev2 = max(prev2, trees[r][n-i]);
            i += 1;
        }
    }

    for c in 0..trees[0].len() {
        let mut i = 0;
        let mut prev1 = -1;
        let n = trees.len() - 1;
        let mut prev2 = -1;
        while i <= n {
            if prev1 < trees[i][c] {
                included[i][c] = true;
            }
            if prev2 < trees[n-i][c] {
                included[n-i][c] = true;
            }
            prev1 = max(prev1, trees[i][c]);
            prev2 = max(prev2, trees[n-i][c]);
            i += 1;
        }
    }

    let mut ans = 0;
    for row in included {
        for v in row {
            if *v {
                ans += 1;
            }
        }
    }

   print!("Part1: {}", ans);
}

fn part2(trees: &Vec<Vec<i32>>) {
    let mut ans = 0;
    for i in 0..trees.len() {
        for j in 0..trees[i].len() {
            let mut cnt1 = 0;
            for v in j+1..trees[i].len() {
                cnt1 += 1;
                if trees[i][j] <= trees[i][v] {
                    break;
                }
            }
            let mut cnt2 = 0;
            for v in (0..j).rev() {
                cnt2 += 1;
                if trees[i][j] <= trees[i][v] {
                    break;
                }
            }
            let mut cnt3 = 0;
            for v in i+1..trees.len() {
                cnt3 += 1;
                if trees[i][j] <= trees[v][j] {
                    break;
                }
            }
            let mut cnt4 = 0;
            for v in (0..i).rev() {
                cnt4 += 1;
                if trees[i][j] <= trees[v][j] {
                    break;
                }
            }
            ans = max(ans, cnt1 * cnt2 * cnt3 * cnt4);
        }
    }
    print!("Part2: {}", ans);
}

fn main() {
    let content = fs::read_to_string("input").expect("File 'input' not found!");
    let mut trees = Vec::new();
    let mut included = Vec::new();
    for line in content.lines() {
        let mut col = Vec::new();
        let mut in_col = Vec::new();
        for tree in line.chars() {
            col.push((tree as i32) - '0' as i32);
            in_col.push(false);
        }
        trees.push(col);
        included.push(in_col);
    }
    
    part1(&trees, &mut included);
    println!("");
    part2(&trees);
}