use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};

struct GraphApp {
    formula: String,
}

impl Default for GraphApp {
    fn default() -> Self {
        Self {
            formula: "sin(x)".to_string(),
        }
    }
}

impl eframe::App for GraphApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Graphing Calculator");

            ui.horizontal(|ui| {
                ui.label("f(x) = ");
                ui.text_edit_singleline(&mut self.formula);
            });

            let points = generate_points(&self.formula);

            Plot::new("plot")
                .view_aspect(2.0)
                .show(ui, |plot_ui| {
                    if let Some(points) = points {
                        plot_ui.line(Line::new(points));
                    }
                });
        });
    }
}

fn generate_points(formula: &str) -> Option<PlotPoints> {
    let expr = formula.parse::<meval::Expr>().ok()?;
    let func = expr.bind("x").ok()?;

    let mut points = Vec::new();

    for i in -100..100 { // TODO: dynamic range
        let x = i as f64 * 0.1; // TODO: dynamic resolution
        let y = func(x);

        if y.is_finite() {
            points.push([x, y]);
        }
    }

    Some(points.into())
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Graph App",
        options,
        Box::new(|_cc| Box::new(GraphApp::default())),
    )
}