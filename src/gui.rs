use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Picture};
use std::path::Path;

pub fn create_window(app: &Application, image_path: &str) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Image Viewer")
        .default_width(800)
        .default_height(600)
        .build();

    let picture = if Path::new(image_path).exists() {
        Picture::for_filename(image_path)
    } else {
        eprintln!("Error: Image file not found: {}", image_path);
        Picture::new()
    };

    picture.set_can_shrink(true);
    picture.set_content_fit(gtk::ContentFit::Cover);

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hexpand(true)
        .vexpand(true)
        .child(&picture)
        .build();

    window.set_child(Some(&scrolled_window));
    window.present();
}
