use std::fs;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    let content = fs::read_to_string("input").expect("File 'input' not found!");
    part1(&content);
    part2(&content);
}

fn part1(content: &str) {
    let mut grid: Vec<Vec<u8>> = content
        .lines()
        .map(|line| line.bytes().collect())
        .collect();

    let mut start = (0, 0);
    let mut end = (0, 0);

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == b'S' {
                start = (i, j);
                grid[i][j] = b'a';
            } else if grid[i][j] == b'E' {
                end = (i, j);
                grid[i][j] = b'z';
            }
        }
    }

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, start))); // (steps, position)

    visited[start.0][start.1] = true;
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some(Reverse((steps, current))) = queue.pop() {
        if current == end {
            println!("Part 1: {}", steps);
            break;
        }

        for &(dx, dy) in &directions {
            let new_x = current.0 as isize + dx;
            let new_y = current.1 as isize + dy;

            if new_x >= 0 && new_x < grid.len() as isize && new_y >= 0 && new_y < grid[0].len() as isize {
                let new_pos = (new_x as usize, new_y as usize);
                if !visited[new_pos.0][new_pos.1] && can_move(grid[current.0][current.1], grid[new_pos.0][new_pos.1]) {
                    visited[new_pos.0][new_pos.1] = true;
                    queue.push(Reverse((steps + 1, new_pos)));
                }
            }
        }
    }
}

fn can_move(from: u8, to: u8) -> bool {
    return from >= to - 1;
}

fn part2(content: &str) {
    let mut grid: Vec<Vec<u8>> = content
        .lines()
        .map(|line| line.bytes().collect())
        .collect();

    let mut end = (0, 0);

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == b'S' {
                grid[i][j] = b'a';
            } else if grid[i][j] == b'E' {
                end = (i, j);
                grid[i][j] = b'z';
            }
        }
    }

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, end))); // (steps, position)

    visited[end.0][end.1] = true;
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some(Reverse((steps, current))) = queue.pop() {
        if grid[current.0][current.1] == b'a' {
            println!("Part 2: {}", steps);
            break;
        }

        for &(dx, dy) in &directions {
            let new_x = current.0 as isize + dx;
            let new_y = current.1 as isize + dy;

            if new_x >= 0 && new_x < grid.len() as isize && new_y >= 0 && new_y < grid[0].len() as isize {
                let new_pos = (new_x as usize, new_y as usize);
                if !visited[new_pos.0][new_pos.1] && can_move(grid[new_pos.0][new_pos.1], grid[current.0][current.1]) {
                    visited[new_pos.0][new_pos.1] = true;
                    queue.push(Reverse((steps + 1, new_pos)));
                }
            }
        }
    }
}