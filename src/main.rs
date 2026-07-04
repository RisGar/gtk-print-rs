use gtk::gio::{Cancellable, File};
use gtk::{Application, PrintDialog, Window, glib};
use gtk::{PrintSetup, prelude::*};
use std::env;

const APP_ID: &str = "org.rishab.gtk-print-rs";

fn main() -> glib::ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file.pdf>", &args[0]);
        return glib::ExitCode::FAILURE;
    }

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(move |app| spawn_dialog(app, &args[1]));

    const EMPTY: [&str; 0] = [];
    app.run_with_args(&EMPTY)
}

fn spawn_dialog(app: &Application, file_path: &str) {
    let file = File::for_path(file_path);
    let dialog = PrintDialog::new();
    let app_clone = app.clone();

    let hold = app.hold();

    gtk::glib::MainContext::default().invoke_local(move || {
        dialog.print_file(
            None::<&Window>,
            None::<&PrintSetup>,
            &file,
            None::<&Cancellable>,
            move |res| {
                match res {
                    Ok(_) => println!("File printing finished (or dialog was closed)."),
                    Err(e) => eprintln!("Failed to print file: {}", e),
                }

                app_clone.quit();
                drop(hold);
            },
        );
    });
}
