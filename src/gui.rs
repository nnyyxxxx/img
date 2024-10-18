use gtk::{prelude::*, Application, ApplicationWindow, Picture, ScrolledWindow};

pub fn create_window(app: &Application, image_path: &str) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Image Viewer")
        .default_width(800)
        .default_height(600)
        .build();

    let picture = Picture::for_filename(image_path);
    picture.set_can_shrink(true);
    picture.set_content_fit(gtk::ContentFit::Cover);

    let scrolled_window = ScrolledWindow::builder()
        .hexpand(true)
        .vexpand(true)
        .child(&picture)
        .build();

    window.set_child(Some(&scrolled_window));
    window.present();
}
