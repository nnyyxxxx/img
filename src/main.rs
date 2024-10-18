mod gui;

use gtk::{prelude::*, Application};
use std::{env, path::Path};

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
        if !Path::new(image_path).exists() {
            eprintln!("Error: file not found: {}", image_path);
            std::process::exit(1);
        }

        gui::create_window(app, image_path);
    });

    application.connect_open(|app, files, _| {
        if let Some(file) = files.first() {
            let path = file.path().unwrap();
            if path.exists() {
                gui::create_window(app, path.to_str().unwrap());
            } else {
                eprintln!("Error: file not found: {}", path.display());
            }
        }
    });

    application.run();
}
