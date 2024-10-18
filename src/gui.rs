use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Image};
use std::path::Path;

pub fn create_window(app: &Application, image_path: &str) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Image Viewer")
        .default_width(800)
        .default_height(600)
        .build();

    let image = if Path::new(image_path).exists() {
        Image::from_file(image_path)
    } else {
        eprintln!("Error: Image file not found: {}", image_path);
        Image::new()
    };

    window.set_child(Some(&image));
    window.present();
}
