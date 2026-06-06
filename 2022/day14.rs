use std::fs;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Coord {
    x: i32,
    y: i32,
}

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");
    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

fn draw_grid(grid: &Vec<Vec<i32>>) {
    println!("");
    for row in grid {
        for col in row {
            let c = match *col {
                0 => '.',
                1 => '#',
                2 => 'O',
                _ => 'X',
            };
            print!("{}", c);
        }
        println!("");
    }
}

fn create_paths_from_input(input: &str) -> (Vec<Vec<Coord>>, i32, i32, i32) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_x = 1000;

    let mut paths = Vec::new();

    for line in input.lines() {
        let path: Vec<Coord> = line.split(" -> ").map(|v| { 
            let mut xy = v.split(',');
            let x = xy.next().unwrap().parse().unwrap();
            let y = xy.next().unwrap().parse().unwrap();

            if x > max_x {
                max_x = x;
            }
            if x < min_x {
                min_x = x;
            }
            if y > max_y {
                max_y = y;
            }

            return Coord{ x, y };
        }).collect();
        paths.push(path);
    }

    return (paths, max_x, max_y, min_x)
}

fn fill_grid(grid: &mut Vec<Vec<i32>>, paths: &Vec<Vec<Coord>>, x_start: i32) {
    for path in paths {
        let mut cur = &path[0];
        let mut i = 1;
        while i < path.len() {
            if cur.x != path[i].x {
                let d = if path[i].x - cur.x > 0 { cur.x..=path[i].x } else { path[i].x..=cur.x };
                for j in d {
                    grid[cur.y as usize][(j - x_start) as usize] = 1;
                }
            } else {
                let d = if path[i].y - cur.y > 0 { cur.y..=path[i].y } else { path[i].y..=cur.y };
                for j in d {
                    grid[j as usize][(cur.x - x_start) as usize] = 1;
                }
            }
            cur = &path[i];
            i += 1;
        }
    }
    let n = grid.len();
    for i in 0..grid[0].len() {
        grid[n-1][i] = 1;
    }
}

fn solve_part1(input: &str) -> i32 {
    let (paths, max_x, max_y, min_x) = create_paths_from_input(&input);

    let x_start = min_x - 1;
    let mut grid: Vec<Vec<i32>> = vec![vec![0; (max_x - min_x) as usize + 3]; max_y as usize + 3];

    fill_grid(&mut grid, &paths, x_start);
    draw_grid(&grid);
    
    let pouring_point = Coord {
        x: 500 - x_start,
        y: 0,
    };

    let mut ans = 0;
    loop {
        let mut cur = pouring_point.clone();
        
        let done = loop {
            if cur.y + 1 > max_y {
                break true;
            }

            if grid[cur.y as usize + 1][cur.x as usize] == 0 {
                cur.y += 1;
            } else if grid[cur.y as usize + 1][cur.x as usize - 1] == 0 {
                cur.y += 1;
                cur.x -= 1;
            } else if grid[cur.y as usize + 1][cur.x as usize + 1] == 0 {
                cur.y += 1;
                cur.x += 1;
            } else {
                ans += 1;
                grid[cur.y as usize][cur.x as usize] = 2;
                break false;
            }
        };

        if done {
            break;
        }
    }

    draw_grid(&grid);

    return ans;
}

fn solve_part2(input: &str) -> i32 {
    let (paths, max_x, max_y, min_x) = create_paths_from_input(&input);

    let x_start = min_x - max_y;
    let mut grid: Vec<Vec<i32>> = vec![vec![0; (max_x - min_x + max_y * 2) as usize + 6]; max_y as usize + 3];

    fill_grid(&mut grid, &paths, x_start);
    draw_grid(&grid);
    
    let pouring_point = Coord {
        x: 500 - x_start,
        y: 0,
    };

    let mut ans = 0;
    loop {
        let mut cur = pouring_point.clone();
        
        if grid[cur.y as usize][cur.x as usize] != 0 {
            break;
        }

        loop {
            if grid[cur.y as usize + 1][cur.x as usize] == 0 {
                cur.y += 1;
            } else if grid[cur.y as usize + 1][cur.x as usize - 1] == 0 {
                cur.y += 1;
                cur.x -= 1;
            } else if grid[cur.y as usize + 1][cur.x as usize + 1] == 0 {
                cur.y += 1;
                cur.x += 1;
            } else {
                ans += 1;
                grid[cur.y as usize][cur.x as usize] = 2;
                break;
            }
        };
    }

    draw_grid(&grid);

    return ans;
}
