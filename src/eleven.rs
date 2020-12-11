use std::fs;

pub fn day_eleven() {
    let mut grid = fs::read_to_string("input/day11.txt")
        .expect("oh no")
        .lines()
        .map(|x| parse(x))
        .collect::<Grid>();

    loop {
        let result = simulate(&grid);
        grid = result.grid;
        if !result.changed {
            println!("found stable grid with {} occupied seats", result.num_occupied);
            break;
        }
    }
}

type Grid = Vec<Vec<Cell>>;

#[derive(Debug)]
enum Cell {
    Floor,
    Empty,
    Occupied,
}

#[derive(Debug)]
struct SimResult {
    grid: Grid,
    changed: bool,
    num_occupied: u32,
}

fn parse(s: &str) -> Vec<Cell> {
    let mut v = Vec::new();
    for c in s.chars() {
        match c {
            '.' => v.push(Cell::Floor),
            'L' => v.push(Cell::Empty),
            '#' => v.push(Cell::Occupied),
            _ => panic!("bad input: {}", c),
        }
    }
    v
}

fn simulate(grid: &Grid) -> SimResult {
    let mut new_grid = Vec::new();
    let mut changed = false;
    let mut num_occupied = 0;

    for (i, row) in grid.iter().enumerate() {
        let mut new_row = Vec::new();
        for (j, cell) in row.iter().enumerate() {
            match cell {
                Cell::Floor => new_row.push(Cell::Floor),
                Cell::Empty => {
                    if number_of_visible_neighbors(grid, i, j) == 0 {
                        new_row.push(Cell::Occupied);
                        num_occupied += 1;
                        changed = true;
                    } else {
                        new_row.push(Cell::Empty);
                    }
                },
                Cell::Occupied => {
                    if number_of_visible_neighbors(grid, i, j) >= 5 {
                        new_row.push(Cell::Empty);
                        changed = true;
                    } else {
                        new_row.push(Cell::Occupied);
                        num_occupied += 1;
                    }
                },
            }
        }
        new_grid.push(new_row);
    }
    SimResult {
        grid: new_grid,
        changed: changed,
        num_occupied: num_occupied,
    }
}

fn number_of_visible_neighbors(grid: &Grid, row: usize, column: usize) -> u8 {
    let mut n = 0;
    //top left
    n += if is_next_visible_occupied(grid, row, column, (-1, -1)) { 1 } else { 0 };
    //top center
    n += if is_next_visible_occupied(grid, row, column, (-1, 0)) { 1 } else { 0 };
    //top right
    n += if is_next_visible_occupied(grid, row, column, (-1, 1)) { 1 } else { 0 };

    //center left
    n += if is_next_visible_occupied(grid, row, column, (0, -1)) { 1 } else { 0 };
    //center right
    n += if is_next_visible_occupied(grid, row, column, (0, 1)) { 1 } else { 0 };

    //bottom left
    n += if is_next_visible_occupied(grid, row, column, (1, -1)) { 1 } else { 0 };
    //bottom center
    n += if is_next_visible_occupied(grid, row, column, (1, 0)) { 1 } else { 0 };
    //bottom right
    n += if is_next_visible_occupied(grid, row, column, (1, 1)) { 1 } else { 0 };

    n
}

fn is_next_visible_occupied(grid: &Grid, r: usize, c: usize, vector: (isize, isize)) -> bool {
    let num_rows = grid.len();
    let num_columns = grid[0].len();
    //this seems silly, but I want them to be mut, but not be &mut usize I think. Not sure how else to do it
    let mut row = r;
    let mut column = c;
    
    loop {
        if let (Some(next_row), Some(next_column)) = (next_index(row, vector.0), next_index(column, vector.1)) {
            if next_row >= num_rows || next_column >= num_columns {
                break;
            }

            match grid[next_row][next_column] {
                Cell::Empty => return false,
                Cell::Floor => (),
                Cell::Occupied => return true,
            }

            row = next_row;
            column = next_column;
        } else {
            break;
        }
    }

    false
}

fn next_index(i: usize, delta: isize) -> Option<usize> {
    if delta < 0 {
        i.checked_sub(delta.wrapping_abs() as usize)
    } else {
        i.checked_add(delta as usize)
    }
}

fn number_of_neighbors(grid: &Grid, row: usize, column: usize) -> u8 {
    let mut n = 0;
    let on_top = row == 0;
    let on_right = column == grid[0].len() - 1;
    let on_bottom = row == grid.len() - 1;
    let on_left = column == 0;

    //top left
    n += if !on_top && !on_left && is_occupied(grid, row - 1, column - 1) { 1 } else { 0 };
    //top center
    n += if !on_top && is_occupied(grid, row - 1, column) { 1 } else { 0 };
    //top right
    n += if !on_top && !on_right && is_occupied(grid, row - 1, column + 1) { 1 } else { 0 };

    //center left
    n += if !on_left && is_occupied(grid, row, column - 1) { 1 } else { 0 };
    //center right
    n += if !on_right && is_occupied(grid, row, column + 1) { 1 } else { 0 };

    //bottom left
    n += if !on_bottom && !on_left && is_occupied(grid, row + 1, column - 1) { 1 } else { 0 };
    //bottom center
    n += if !on_bottom && is_occupied(grid, row + 1, column) { 1 } else { 0 };
    //bottom right
    n += if !on_bottom && !on_right && is_occupied(grid, row + 1, column + 1) { 1 } else { 0 };

    n
}

fn is_occupied(grid: &Grid, row: usize, column: usize) -> bool {
    match grid[row][column] {
        Cell::Occupied => true,
        _ => false
    }
}
