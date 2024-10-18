mod gui;

use gtk::prelude::*;
use gtk::Application;
use std::env;

fn main() {
    let application = Application::builder()
        .application_id("nnyyxxxx.img")
        .flags(gtk::gio::ApplicationFlags::HANDLES_OPEN)
        .build();

    application.connect_activate(|app| {
        let args: Vec<String> = env::args().collect();
        if args.len() != 2 {
            eprintln!("Usage: {} <path_to_image>", args[0]);
            std::process::exit(1);
        }

        let image_path = &args[1];
        gui::create_window(app, image_path);
    });

    application.connect_open(|app, files, _| {
        if let Some(file) = files.first() {
            gui::create_window(app, file.path().unwrap().to_str().unwrap());
        }
    });

    application.run();
}
