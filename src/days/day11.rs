pub fn part1() {
    let (max_power, x_max, y_max) = run_grid(5177);

    println!("{} at {},{}", max_power, x_max, y_max);
}

pub fn part2() {
    let (max_power, x_max, y_max, square_size_max) = run_grid_2(5177);

    println!("{} at {},{},{}", max_power, x_max, y_max, square_size_max);
}

fn power_level(x: usize, y: usize, serial_number: usize) -> isize {
    let rack_id = x + 10;
    let mut value = (rack_id * y) as isize;
    value += serial_number as isize;
    value *= rack_id as isize;
    value = (value / 100) % 10;
    value -= 5;

    value
}

fn create_grid(serial_number: usize) -> [[isize; 300]; 300] {
    let mut grid = [[0; 300]; 300];

    for (y, row) in grid.iter_mut().enumerate() {
        for (x, cell) in row.iter_mut().enumerate() {
            *cell = power_level(x + 1, y + 1, serial_number);
        }
    }

    grid
}

fn get_square_value(grid: &[[isize; 300]; 300], square_size: usize) -> (isize, usize, usize) {
    let (mut max_power, mut x_max, mut y_max) = (0, 0, 0);

    for y in 0..grid.len() - square_size {
        for x in 0..grid[y].len() - square_size {
            let mut value = 0;

            for check_row in grid.iter().skip(y).take(square_size) {
                for check_cell in check_row.iter().skip(x).take(square_size) {
                    value += check_cell;
                }
            }

            if value > max_power {
                max_power = value;
                x_max = x + 1;
                y_max = y + 1;
            }
        }
    }

    (max_power, x_max, y_max)
}

fn run_grid(serial_number: usize) -> (isize, usize, usize) {
    let grid = create_grid(serial_number);

    get_square_value(&grid, 3)
}

fn run_grid_2(serial_number: usize) -> (isize, usize, usize, usize) {
    let grid = create_grid(serial_number);

    let (mut max_power, mut x_max, mut y_max, mut square_size_max) = (0, 0, 0, 0);
    
    for square_size in 1..=300 {
        let (power, x, y) = get_square_value(&grid, square_size);

        if power > max_power {
            max_power = power;
            x_max = x;
            y_max = y;
            square_size_max = square_size;
        }
    }

    (max_power, x_max, y_max, square_size_max)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(3, 5, 8, 4)]
    #[case(122, 79, 57, -5)]
    #[case(217, 196, 39, 0)]
    #[case(101, 153, 71, 4)]
    fn test1(
        #[case] x: usize,
        #[case] y: usize,
        #[case] serial_number: usize,
        #[case] expected_power_level: isize,
    ) {
        assert_eq!(expected_power_level, power_level(x, y, serial_number));
    }

    #[rstest]
    fn test2() {
        let (max_power, x_max, y_max) = run_grid(42);

        assert_eq!(30, max_power);
        assert_eq!(21, x_max);
        assert_eq!(61, y_max);
    }
}
