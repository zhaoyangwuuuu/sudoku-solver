use eframe::egui;

pub struct SudokuSolver {
    pub grid: Vec<Vec<String>>,
    show_unsolvable_popup: bool,
    has_error: bool,
}

impl SudokuSolver {
    pub fn new() -> Self {
        Self {
            grid: vec![vec![String::from(""); 9]; 9],
            show_unsolvable_popup: false,
            has_error: false,
        }
    }

    pub fn draw_grid(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("sudoku_grid").striped(true).show(ui, |ui| {
            for row in 0..9 {
                for col in 0..9 {
                    let cell = &mut self.grid[row][col];
                    let response = ui.add(egui::TextEdit::singleline(cell).desired_width(50.0));

                    if response.changed() {}

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
        self.check_for_errors();

        if self.has_error {
            self.show_unsolvable_message();
            false
        } else {
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

    pub fn show_unsolvable_message(&mut self) {
        self.show_unsolvable_popup = true;
    }

    pub fn draw_unsolvable_popup(&mut self, ctx: &egui::Context) {
        if self.show_unsolvable_popup {
            egui::Window::new("Unsolvable Puzzle")
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO) // Center the popup
                .show(ctx, |ui| {
                    ui.label("This Sudoku puzzle is not solvable with the current inputs.");
                    if ui.button("OK").clicked() {
                        self.show_unsolvable_popup = false;
                    }
                });
        }
    }

    pub fn reset_grid(&mut self) {
        for row in self.grid.iter_mut() {
            for cell in row.iter_mut() {
                *cell = String::new();
            }
        }
        self.show_unsolvable_popup = false;
    }

    fn is_row_valid(&self, row: usize) -> bool {
        let mut seen = [false; 9];
        for col in 0..9 {
            if let Ok(num) = self.grid[row][col].parse::<usize>() {
                if num == 0 || num > 9 || seen[num - 1] {
                    return false;
                }
                seen[num - 1] = true;
            }
        }
        true
    }

    fn is_col_valid(&self, col: usize) -> bool {
        let mut seen = [false; 9];
        for row in 0..9 {
            if let Ok(num) = self.grid[row][col].parse::<usize>() {
                if num == 0 || num > 9 || seen[num - 1] {
                    return false;
                }
                seen[num - 1] = true;
            }
        }
        true
    }

    fn is_subgrid_valid(&self, start_row: usize, start_col: usize) -> bool {
        let mut seen = [false; 9];
        for row in 0..3 {
            for col in 0..3 {
                if let Ok(num) = self.grid[start_row + row][start_col + col].parse::<usize>() {
                    if num == 0 || num > 9 {
                        continue;
                    }
                    if seen[num - 1] {
                        return false;
                    }
                    seen[num - 1] = true;
                }
            }
        }
        true // No duplicates found
    }
    pub fn check_for_errors(&mut self) {
        self.has_error = false; // Reset the error flag

        for i in 0..9 {
            if !self.is_row_valid(i) || !self.is_col_valid(i) {
                self.has_error = true;
                return;
            }
        }

        for row in (0..9).step_by(3) {
            for col in (0..9).step_by(3) {
                if !self.is_subgrid_valid(row, col) {
                    self.has_error = true;
                    return;
                }
            }
        }
    }
}

impl Default for SudokuSolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_validity() {
        let mut app = SudokuSolver::new();
        app.grid[0] = vec![
            "5".to_string(),
            "3".to_string(),
            "".to_string(),
            "".to_string(),
            "7".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ];
        assert!(app.is_row_valid(0) == true);

        app.grid[0][1] = "5".to_string();
        assert!(app.is_row_valid(0) == false);
    }

    #[test]
    fn test_col_validity() {
        let mut app = SudokuSolver::new();
        app.grid[0][0] = "5".to_string();
        app.grid[1][0] = "6".to_string();
        assert!(app.is_col_valid(0) == true);

        app.grid[2][0] = "5".to_string();
        assert!(app.is_col_valid(0) == false);
    }

    #[test]
    fn test_subgrid_validity() {
        let mut app = SudokuSolver::new();
        app.grid[0][0] = "5".to_string();
        app.grid[1][1] = "5".to_string();
        assert!(app.is_subgrid_valid(0, 0) == false);

        app.grid[0][2] = "5".to_string();
        assert!(app.is_subgrid_valid(0, 0) == false);
    }
}
