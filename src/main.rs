use id3modifier::load_tags;

mod id3modifier;

// fn main() {
//     let native_options = eframe::NativeOptions::default();
//     eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(MP3ID3Gui::new(cc))));
// }

fn main() {
    let _path1 = "E:/Voice/RJ358440/RJ358440.mp3";
    let path2 = "E:/Voice/RJ337611/RJ337611.mp3";
    let _test = load_tags(path2);
}
