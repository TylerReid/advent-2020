use std::fs;
use std::convert::TryInto;
use std::cmp::max;

pub fn day_seventeen() {
    let input = fs::read_to_string("input/day17.txt").expect("oh no");

    let mut active_cubes = Vec::<(i64, i64, i64, i64)>::new();

    for (i, s) in input.lines().enumerate() {
        for (j, c) in s.chars().enumerate() {
            if c == '#' {
                active_cubes.push((i.try_into().unwrap(), j.try_into().unwrap(), 0, 0));
            }
        }
    }

    for _ in 0..6 {
        active_cubes = simulate(&active_cubes, &get_range(&active_cubes));
    }

    println!("{}", active_cubes.len());
}

fn simulate(grid: &Vec<(i64, i64, i64, i64)>, range: &(i64, i64, i64, i64)) -> Vec<(i64, i64, i64, i64)> {
    let mut active_cubes = Vec::new();
    for x in -range.0-1..=range.0+1 {
        for y in -range.1-1..=range.1+1 {
            for z in -range.2-1..=range.2+1 {
                for w in -range.3-1..=range.3+1 {
                    let neighbors = number_of_neighbors(grid, (x, y, z, w));
                    if neighbors == 2 {
                        let is_active = grid.contains(&(x, y, z, w));
                        if is_active {
                            active_cubes.push((x, y, z, w));
                        }
                    }
                    if neighbors == 3 {
                        active_cubes.push((x, y, z, w));
                    }
                }
            }
        }
    }

    active_cubes
}

fn number_of_neighbors(grid: &Vec::<(i64, i64, i64, i64)>, cube: (i64, i64, i64, i64)) -> u8 {
    let mut neighbors = 0;

    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                for w in -1..=1 {
                    if x == 0 && y == 0 && z == 0 && w == 0 {
                        continue;
                    }
    
                    if grid.contains(&(cube.0 + x, cube.1 + y, cube.2 + z, cube.3 + w)) {
                        neighbors += 1
                    }
                }
            }
        } 
    }

    neighbors
}

fn get_range(grid: &Vec::<(i64, i64, i64, i64)>) -> (i64, i64, i64, i64) {
    let mut range = (0, 0, 0, 0);
    for cube in grid.iter() {
        range.0 = std::cmp::max(range.0, cube.0.abs());
        range.1 = std::cmp::max(range.1, cube.1.abs());
        range.2 = std::cmp::max(range.2, cube.2.abs());
        range.3 = std::cmp::max(range.3, cube.3.abs());
    }
    range
}
