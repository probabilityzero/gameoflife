use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::Rng;
use std::io::{stdout, Write};
use std::time::Duration;
use std::io;

#[derive(Clone, Debug)]
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<bool>>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {
            width,
            height,
            cells: vec![vec![false; width]; height],
        }
    }

    fn random(width: usize, height: usize) -> Grid {
        let mut rng = rand::thread_rng();
        let mut cells = vec![vec![false; width]; height];
        for row in &mut cells {
            for cell in row {
                *cell = rng.gen_bool(0.3); // 30% chance of a cell being alive
            }
        }
        Grid { width, height, cells }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        // Toroidal wrapping using wrapping_add and wrapping_rem
        let wrapped_x = x.wrapping_add(self.width) % self.width;
        let wrapped_y = y.wrapping_add(self.height) % self.height;
        self.cells[wrapped_y][wrapped_x]
    }

    fn set(&mut self, x: usize, y: usize, value: bool) {
        let wrapped_x = x.wrapping_add(self.width) % self.width;
        let wrapped_y = y.wrapping_add(self.height) % self.height;
        self.cells[wrapped_y][wrapped_x] = value;
    }

    fn count_live_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if self.get((x as isize + dx) as usize, (y as isize + dy) as usize) {
                    count += 1;
                }
            }
        }
        count
    }

    fn next_generation(&self) -> Grid {
        let mut next_grid = Grid::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let live_neighbors = self.count_live_neighbors(x, y);
                let current_state = self.get(x, y);
                let next_state = match (current_state, live_neighbors) {
                    (true, 2..=3) => true,  // Survival
                    (true, _) => false,     // Underpopulation or Overpopulation
                    (false, 3) => true,    // Reproduction
                    (false, _) => false,
                };
                next_grid.set(x, y, next_state);
            }
        }
        next_grid
    }

    fn render(&self) {
        let mut stdout = stdout();

        for y in 0..self.height {
            for x in 0..self.width {
                execute!(
                    stdout,
                    MoveTo(x as u16, y as u16),
                    match self.get(x, y) {
                        true => SetBackgroundColor(Color::White),
                        false => SetBackgroundColor(Color::Black),
                    },
                    Print(" "), // Use a space to ensure consistent cell size
                    ResetColor
                )
                .unwrap();
            }
        }
        stdout.flush().unwrap();
    }
}

fn main() -> io::Result<()> {
    // --- Setup ---
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;
    crossterm::terminal::enable_raw_mode()?;

    let (width, height) = crossterm::terminal::size()?;
    let width = width as usize;
    let height = height as usize;

    // Adjust the size, subtracting one to use for the amount of rows
    let mut grid = Grid::random(width, height - 1);

    // --- Main Loop ---
    loop {
        grid.render();
        grid = grid.next_generation();

        // --- Input Handling (check for 'q' to quit) ---
        if poll(Duration::from_millis(100))? {
            // Check for input every 100ms
            if let Event::Key(key_event) = read()? {
                if key_event.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
        execute!(
            stdout,
            MoveTo(0, height as u16 - 1),
            Print(" Q Quit ")
        )?;
    }

    // --- Cleanup ---
    execute!(stdout, Show, LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}