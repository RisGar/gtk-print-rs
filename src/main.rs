use gtk::prelude::*;
use gtk::{Application, glib};

const APP_ID: &str = "org.rishab.GtkPrintRs";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.run()
}
