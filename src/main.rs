mod config;
mod ui;
mod vdrive;

use relm4::{RelmApp, gtk};

fn main() {
    // Initialize GTK
    gtk::init().expect("Failed to initialize GTK");

    // Set up libadwaita
    libadwaita::init().expect("Failed to initialize libadwaita");

    // Create and run the app
    let app = RelmApp::new("com.kinesis.configurator");
    app.run::<ui::AppModel>(());
}
