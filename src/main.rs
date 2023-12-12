mod sudoku_solver;

use eframe::egui;
use sudoku_solver::SudokuSolver;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([480.0, 320.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Sudoku",
        options,
        Box::new(|_| Box::<SudokuSolver>::default()),
    )
}

impl eframe::App for SudokuSolver {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Sudoku Solver");
            ui.add_space(10.0);
            self.draw_grid(ui);
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button("Reset").clicked() {
                    self.reset_grid();
                }
                if ui.button("Solve Sudoku").clicked() {
                    let solvable = self.solve_sudoku();
                    if !solvable {
                        self.show_unsolvable_message();
                    }
                }
            });
            self.draw_unsolvable_popup(ctx);
        });
    }
}
