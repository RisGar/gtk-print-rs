#[cfg(target_os = "linux")]
mod platform {
    use gtk::gio::File;
    use gtk::prelude::*;
    use gtk::{Application, PrintDialog, PrintSetup, Window, glib};
    use std::env;

    const APP_ID: &str = "org.rishab.print-cli-rs";

    pub fn run() {
        let args: Vec<String> = env::args().collect();
        if args.len() != 2 {
            eprintln!("Usage: {} <file.pdf>", &args[0]);
            std::process::exit(1);
        }

        let app = Application::builder().application_id(APP_ID).build();

        app.connect_activate(move |app| spawn_dialog(app, &args[1]));

        app.run_with_args::<&str>(&[]);
    }

    fn spawn_dialog(app: &Application, file_path: &str) {
        let file = File::for_path(file_path);
        let dialog = PrintDialog::new();
        let app_clone = app.clone();

        let hold = app.hold();

        gtk::glib::MainContext::default().spawn_local(async move {
            match dialog
                .print_file_future(None::<&Window>, None::<&PrintSetup>, &file)
                .await
            {
                Ok(_) => println!("File printing finished (or dialog was closed)."),
                Err(e) => eprintln!("Failed to print file: {}", e),
            }

            app_clone.quit();
            drop(hold);
        });
    }
}

#[cfg(target_os = "macos")]
mod platform {
    use objc2::AnyThread;
    use objc2::MainThreadMarker;
    use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy, NSPrintInfo};
    use objc2_foundation::{NSString, NSURL};
    use objc2_pdf_kit::{PDFDocument, PDFPrintScalingMode};
    use std::env;

    pub fn run() {
        let args: Vec<String> = env::args().collect();
        if args.len() != 2 {
            eprintln!("Usage: {} <file.pdf>", &args[0]);
            std::process::exit(1);
        }

        let file_path = &args[1];

        let mtm = MainThreadMarker::new().expect("Must run on main thread");

        unsafe {
            let app = NSApplication::sharedApplication(mtm);
            // Accessory makes the app run without a dock icon, which helps tiling WMs like AeroSpace treat it as a floating popup
            app.setActivationPolicy(NSApplicationActivationPolicy::Accessory);

            #[allow(deprecated)]
            app.activateIgnoringOtherApps(true);

            let path = NSString::from_str(file_path);
            let url = NSURL::fileURLWithPath(&path);

            let pdf_doc = PDFDocument::initWithURL(PDFDocument::alloc(), &url);

            if let Some(doc) = pdf_doc {
                let print_info = NSPrintInfo::sharedPrintInfo();

                let print_op = doc.printOperationForPrintInfo_scalingMode_autoRotate(
                    Some(&print_info),
                    PDFPrintScalingMode::PageScaleDownToFit,
                    true,
                    mtm,
                );

                if let Some(op) = print_op {
                    op.setShowsPrintPanel(true);
                    op.runOperation();
                    println!("File printing finished (or dialog was closed).");
                } else {
                    eprintln!("Failed to create print operation.");
                }
            } else {
                eprintln!("Failed to load PDF file.");
            }
        }
    }
}

fn main() {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    platform::run();

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    eprintln!("Unsupported operating system");
}
