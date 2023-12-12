use eframe::egui;

pub struct SudokuSolver {
    pub grid: Vec<Vec<String>>,
}

impl SudokuSolver {
    pub fn new() -> Self {
        Self {
            grid: vec![vec![String::from(""); 9]; 9],
        }
    }

    pub fn draw_grid(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("sudoku_grid").striped(true).show(ui, |ui| {
            for row in 0..9 {
                for col in 0..9 {
                    let cell = &mut self.grid[row][col];
                    ui.add(egui::TextEdit::singleline(cell).desired_width(50.0));

                    if (col + 1) % 3 == 0 && col != 8 {
                        ui.add_space(16.0);
                    }
                }
                ui.end_row();

                if (row + 1) % 3 == 0 && row != 8 {
                    ui.add_space(10.0);
                    ui.end_row();
                }
            }
        });
    }

    pub fn solve_sudoku(&mut self) -> bool {
        if let Some((row, col)) = self.find_unassigned_location() {
            for num in 1..=9 {
                if self.is_safe(row, col, num) {
                    self.grid[row][col] = num.to_string();
                    if self.solve_sudoku() {
                        return true;
                    }
                    self.grid[row][col] = String::new();
                }
            }
            false
        } else {
            true
        }
    }

    fn find_unassigned_location(&self) -> Option<(usize, usize)> {
        for row in 0..9 {
            for col in 0..9 {
                if self.grid[row][col].is_empty() {
                    return Some((row, col));
                }
            }
        }
        None
    }

    fn is_safe(&self, row: usize, col: usize, num: u32) -> bool {
        !self.used_in_row(row, num)
            && !self.used_in_col(col, num)
            && !self.used_in_box(row - row % 3, col - col % 3, num)
    }

    fn used_in_row(&self, row: usize, num: u32) -> bool {
        self.grid[row].iter().any(|cell| cell == &num.to_string())
    }

    fn used_in_col(&self, col: usize, num: u32) -> bool {
        self.grid.iter().any(|row| row[col] == num.to_string())
    }

    fn used_in_box(&self, box_start_row: usize, box_start_col: usize, num: u32) -> bool {
        for row in 0..3 {
            for col in 0..3 {
                if self.grid[row + box_start_row][col + box_start_col] == num.to_string() {
                    return true;
                }
            }
        }
        false
    }
}

impl Default for SudokuSolver {
    fn default() -> Self {
        Self::new()
    }
}
