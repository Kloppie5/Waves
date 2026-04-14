use eframe::egui;
use egui::{Color32, Pos2, Stroke};
use rand::Rng;

struct FunctionEntry {
    formula: String,
    color: Color32,
}

struct GraphApp {
    functions: Vec<FunctionEntry>,
    rot_x: f64,
    rot_y: f64,
    zoom: f64,
}

impl Default for GraphApp {
    fn default() -> Self {
        Self {
            functions: vec![FunctionEntry {
                formula: "sin(x) * cos(y)".to_string(),
                color: random_color(),
            }],
            rot_x: 0.6,
            rot_y: 0.8,
            zoom: 100.0,
        }
    }
}

impl eframe::App for GraphApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("controls").show(ctx, |ui| {
            ui.heading("Functions");

            if ui.button("Add Function").clicked() {
                self.functions.push(FunctionEntry {
                    formula: "sin(x*y)".to_string(),
                    color: random_color(),
                });
            }

            ui.add(egui::Slider::new(&mut self.rot_x, 0.0..=6.28).text("Rot X"));
            ui.add(egui::Slider::new(&mut self.rot_y, 0.0..=6.28).text("Rot Y"));
            ui.add(egui::Slider::new(&mut self.zoom, 20.0..=400.0).text("Zoom"));

            ui.separator();

            let mut remove = None;

            for (i, f) in self.functions.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("f{}:", i + 1));
                    ui.text_edit_singleline(&mut f.formula);

                    ui.color_edit_button_srgba(&mut f.color);

                    if ui.button("❌").clicked() {
                        remove = Some(i);
                    }
                });
            }

            if let Some(i) = remove {
                self.functions.remove(i);
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let rect = ui.available_rect_before_wrap();
            let center = rect.center();
            let painter = ui.painter();

            draw_axis(painter, center, (0.0, 0.0, 0.0), (2.0, 0.0, 0.0), self.rot_x, self.rot_y, self.zoom, Color32::RED);
            draw_axis(painter, center, (0.0, 0.0, 0.0), (0.0, 2.0, 0.0), self.rot_x, self.rot_y, self.zoom, Color32::GREEN);
            draw_axis(painter, center, (0.0, 0.0, 0.0), (0.0, 0.0, 2.0), self.rot_x, self.rot_y, self.zoom, Color32::BLUE);

            for func in &self.functions {
                draw_function(
                    painter,
                    center,
                    &func.formula,
                    func.color,
                    self.rot_x,
                    self.rot_y,
                    self.zoom,
                );
            }
        });
    }
}

fn project(x: f64, y: f64, z: f64, rx: f64, ry: f64, zoom: f64) -> Pos2 {
    let (sy, cy) = ry.sin_cos();
    let x1 = cy * x + sy * z;
    let z1 = -sy * x + cy * z;

    let (sx, cx) = rx.sin_cos();
    let y1 = cx * y - sx * z1;

    let scale = 1.0 / (1.0 + z1 * 0.2);

    Pos2::new(
        (x1 * zoom * scale) as f32,
        (y1 * zoom * scale) as f32,
    )
}

fn draw_function(
    painter: &egui::Painter,
    center: Pos2,
    formula: &str,
    color: Color32,
    rx: f64,
    ry: f64,
    zoom: f64,
) {
    let expr = match formula.parse::<meval::Expr>() {
        Ok(e) => e,
        Err(_) => return,
    };

    let func = match expr.bind2("x", "y") {
        Ok(f) => f,
        Err(_) => return,
    };

    for xi in -40..40 {
        for yi in -40..40 {
            let x = xi as f64 * 0.2;
            let y = yi as f64 * 0.2;
            let z = func(x, y);

            if z.is_finite() {
                let p = project(x, y, z, rx, ry, zoom);
                painter.circle_filled(center + p.to_vec2(), 1.5, color);
            }
        }
    }
}

fn draw_axis(
    painter: &egui::Painter,
    center: Pos2,
    start: (f64, f64, f64),
    end: (f64, f64, f64),
    rx: f64,
    ry: f64,
    zoom: f64,
    color: Color32,
) {
    let steps = 50;

    let mut prev: Option<Pos2> = None;

    for i in 0..=steps {
        let t = i as f64 / steps as f64;

        let x = start.0 + (end.0 - start.0) * t;
        let y = start.1 + (end.1 - start.1) * t;
        let z = start.2 + (end.2 - start.2) * t;

        let p = project(x, y, z, rx, ry, zoom);

        if let Some(prev_p) = prev {
            painter.line_segment(
                [center + prev_p.to_vec2(), center + p.to_vec2()],
                Stroke::new(2.0, color),
            );
        }

        prev = Some(p);
    }
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
        "Waves 3D Engine",
        options,
        Box::new(|_cc| Box::new(GraphApp::default())),
    )
}