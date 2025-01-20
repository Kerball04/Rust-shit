use eframe::egui;
use egui::{CentralPanel, Context, TextEdit, Ui};

#[derive(Default)]
struct CalculatorApp {
    input: String,
    result: String,
}

impl CalculatorApp {
    fn calculate(&mut self, operation: char) {
        let mut parts = self.input.split_whitespace();
        if let (Some(lhs), Some(rhs)) = (parts.next(), parts.next()) {
            if let (Ok(lhs), Ok(rhs)) = (lhs.parse::<f64>(), rhs.parse::<f64>()) {
                let result = match operation {
                    '+' => lhs + rhs,
                    '-' => lhs - rhs,
                    '*' => lhs * rhs,
                    '/' => {
                        if rhs != 0.0 {
                            lhs / rhs
                        } else {
                            self.result = "Error: Division by zero".to_string();
                            return;
                        }
                    }
                    '^' => lhs.powf(rhs),
                    _ => 0.0,
                };
                self.result = format!("Result: {:.2}", result);
            } else {
                self.result = "Error: Invalid numbers".to_string();
            }
        } else {
            self.result = "Error: Provide two numbers separated by space".to_string();
        }
    }

    fn render_buttons(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button("+").on_hover_text("Add").clicked() {
                self.calculate('+');
            }
            if ui.button("-").on_hover_text("Subtract").clicked() {
                self.calculate('-');
            }
            if ui.button("*").on_hover_text("Multiply").clicked() {
                self.calculate('*');
            }
            if ui.button("/").on_hover_text("Divide").clicked() {
                self.calculate('/');
            }
            if ui.button("^").on_hover_text("Power").clicked() {
                self.calculate('^');
            }
        });
    }
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Advanced Calculator");
            ui.add_space(10.0);
            ui.label("Enter two numbers separated by space:");
            ui.add(TextEdit::singleline(&mut self.input).hint_text("e.g., 3.5 2.1"));

            ui.add_space(10.0);
            self.render_buttons(ui);

            ui.add_space(20.0);
            if !self.result.is_empty() {
                ui.label(egui::RichText::new(&self.result).strong());
            }

            ui.add_space(20.0);
            if ui.button("Quit").on_hover_text("Close the application").clicked() {
                std::process::exit(0);
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Advanced Calculator",
        options,
        Box::new(|_| Ok(Box::new(CalculatorApp::default()))),
    )
}
