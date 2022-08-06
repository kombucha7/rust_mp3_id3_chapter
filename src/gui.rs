use eframe::egui;
use egui::Ui;



fn chapter(ui: &mut Ui, chapter_name: &mut String, start_time: &mut String, end_time: &mut String, chapter_number: &mut i32) {
    let chapter_label = format!("Chapter {}", chapter_number);
    ui.label(chapter_label);

    ui.add(egui::TextEdit::singleline(chapter_name));

    ui.add(egui::TextEdit::singleline(start_time));

    ui.add(egui::TextEdit::singleline(end_time));
    
}

#[derive(Default)]
pub struct MyEguiApp {}

impl MyEguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let &mut chapter_name:String;
       egui::CentralPanel::default().show(ctx, |ui| {
           chapter(ui, &mut String::from("test"), &mut String::from("startTime"), &mut String::from("endTime"), &mut 1)
       });
   }
}