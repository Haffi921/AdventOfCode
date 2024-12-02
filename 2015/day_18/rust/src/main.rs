const GRID_SIZE: usize = 100;

#[derive(Clone)]
struct LightGrid {
    grid: [[bool; GRID_SIZE]; GRID_SIZE],
    pinned: Vec<(usize, usize)>,
}

impl LightGrid {
    fn new(input: &str) -> Self {
        LightGrid {
            grid: input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c == '#')
                        .collect::<Vec<_>>()
                        .try_into()
                        .expect(&format!("Each line must be {} chars", GRID_SIZE))
                })
                .collect::<Vec<[bool; GRID_SIZE]>>()
                .try_into()
                .expect(&format!("Input must be {}x{}", GRID_SIZE, GRID_SIZE)),
            pinned: Vec::new(),
        }
    }

    fn pin(&mut self, x: usize, y: usize) {
        self.pinned.push((x, y));
        self.grid[y][x] = true;
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();

        let mut x_coords = Vec::with_capacity(3);
        let mut y_coords = Vec::with_capacity(3);
        if x > 0 {
            x_coords.push(x - 1);
        }
        if x < GRID_SIZE - 1 {
            x_coords.push(x + 1);
        }
        x_coords.push(x);

        if y > 0 {
            y_coords.push(y - 1);
        }
        if y < GRID_SIZE - 1 {
            y_coords.push(y + 1);
        }
        y_coords.push(y);

        for x_bour in x_coords.iter() {
            for y_bour in y_coords.iter() {
                if x_bour != &x || y_bour != &y {
                    neighbors.push((*x_bour, *y_bour));
                }
            }
        }

        neighbors
    }

    fn should_be_on(&self, x: usize, y: usize) -> bool {
        if self.pinned.contains(&(x, y)) {
            return true;
        }

        let neighbors = self.get_neighbors(x, y);
        let on_neighbors = neighbors.iter().filter(|(x, y)| self.grid[*y][*x]).count();
        self.grid[y][x] && (on_neighbors == 2 || on_neighbors == 3)
            || !self.grid[y][x] && on_neighbors == 3
    }

    fn step(&mut self) {
        let mut new_grid = self.grid.clone();
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                new_grid[y][x] = self.should_be_on(x, y);
            }
        }
        self.grid = new_grid;
    }

    fn step_n(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }

    #[allow(dead_code)]
    fn count_on(&self) -> usize {
        self.grid.iter().flatten().filter(|&&b| b).count()
    }

    #[allow(dead_code)]
    fn render(&self) {
        println!(
            "{}",
            self.grid
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|&b| if b { '#' } else { '.' })
                        .collect::<String>()
                })
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
}

fn part_1(grid: &mut LightGrid) -> usize {
    grid.step_n(100);
    grid.count_on()
}

fn part_2(grid: &mut LightGrid) -> usize {
    grid.pin(0, 0);
    grid.pin(0, GRID_SIZE - 1);
    grid.pin(GRID_SIZE - 1, 0);
    grid.pin(GRID_SIZE - 1, GRID_SIZE - 1);
    grid.step_n(100);
    grid.count_on()
}

fn main() {
    let grid = LightGrid::new(include_str!("../../input.txt"));
    println!("{}", part_1(&mut grid.clone()));
    println!("{}", part_2(&mut grid.clone()));
}
