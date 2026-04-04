use eframe::egui;
use egui::Color32;
use egui_plot::{Legend, Line, Plot, PlotPoints};
use rand::Rng;

struct FunctionEntry {
    formula: String,
    color: Color32,
}

struct GraphApp {
    functions: Vec<FunctionEntry>,
}

impl Default for GraphApp {
    fn default() -> Self {
        Self {
            functions: vec![FunctionEntry {
                formula: "sin(x)".to_string(),
                color: random_color(),
            }],
        }
    }
}

impl eframe::App for GraphApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Graphing Calculator");

            if ui.button("Add Function").clicked() {
                self.functions.push(FunctionEntry {
                    formula: "x".to_string(),
                    color: random_color(),
                });
            }

            ui.separator();

            let mut remove_index = None;

            for (i, func) in self.functions.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("f{}:", i + 1));

                    ui.label("Formula:");
                    ui.text_edit_singleline(&mut func.formula);

                    ui.color_edit_button_srgba(&mut func.color);

                    if ui.button("❌").clicked() {
                        remove_index = Some(i);
                    }
                });
            }

            if let Some(i) = remove_index {
                self.functions.remove(i);
            }

            ui.separator();

            Plot::new("plot")
                .legend(Legend::default())
                .view_aspect(2.0)
                .show(ui, |plot_ui| {
                    for (i, func) in self.functions.iter().enumerate() {
                        if let Some(points) = generate_points(&func.formula) {
                            let label = if i == 0 {
                                "f(x)".to_string()
                            } else {
                                format!("f{}(x)", i + 1)
                            };

                            plot_ui.line(
                                Line::new(points)
                                    .color(func.color)
                                    .name(label),
                            );
                        }
                    }
                });
        });
    }
}

fn generate_points(formula: &str) -> Option<PlotPoints> {
    let expr = formula.parse::<meval::Expr>().ok()?;
    let func = expr.bind("x").ok()?;

    let mut points = Vec::new();

    for i in -100..100 {
        let x = i as f64 * 0.1;
        let y = func(x);
        if y.is_finite() {
            points.push([x, y]);
        }
    }

    Some(points.into())
}

fn random_color() -> Color32 {
    let mut rng = rand::thread_rng();
    Color32::from_rgb(
        rng.gen_range(50..250),
        rng.gen_range(50..250),
        rng.gen_range(50..250),
    )
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Graph App",
        options,
        Box::new(|_cc| Box::new(GraphApp::default())),
    )
}