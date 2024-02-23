// GTK crates
use adw::prelude::*;
use adw::*;
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)

// application crates
/// first setup crates
use crate::first_setup::*;

pub fn build_ui(app: &adw::Application) {
    // setup glib
    gtk::glib::set_prgname(Some("PikaOS First Setup"));
    glib::set_application_name("PikaOS First Setup");

    // create the main Application window
    let window = adw::ApplicationWindow::builder()
        // The text on the titlebar
        .title("PikaOS First Setup")
        // link it to the application "app"
        .application(app)
        // Add the box called "window_box" to it
        // Application icon
        .icon_name("com.github.pikaos-linux.pikawelcome")
        // Minimum Size/Default
        .width_request(700)
        .height_request(500)
        // Hide window instead of destroy
        .hide_on_close(true)
        .deletable(false)
        // Startup
        .startup_id("com.github.pikaos-linux.pikawelcome")
        // build the window
        .build();

    first_setup(&window);

    // show the window
    window.present()
}
