use gtk::prelude::*;
use gtk::{gio, Application};

mod gtk_window;

const APP_ID: &str = "com.rouhim.binvec";

fn main() {
    // Register and include resources
    gio::resources_register_include!("compiled.gresource").expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create new window and present it
    let window = gtk_window::AppWindow::new(app);
    window.present();
}
