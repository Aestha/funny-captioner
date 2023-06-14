use eframe::{egui, run_native};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions{
        initial_window_size: Some(egui::Vec2 {x:320.0, y:240.0}),
        ..Default::default()  
    };
    eframe::run_native(
        "my cheese", 
        options, 
        Box::new(|_cc| Box::<MyApp>::default())
    )
    
}

struct MyApp
{
    name: String,
}

impl Default for MyApp
{
    fn default() -> Self {
        Self { name: "cheese".to_owned() }
    }
}

impl eframe::App for MyApp
{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello World!");
         });
    }
}