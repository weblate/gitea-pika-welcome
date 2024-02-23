// GTK crates
use adw::prelude::*;
use adw::*;
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)

// application crates
/// first setup crates
use crate::config::*;
use crate::first_setup::*;

pub fn build_ui(app: &adw::Application) {
    // setup glib
    gtk::glib::set_prgname(Some(t!("app_name").to_string()));
    glib::set_application_name(&t!("app_name").to_string());

    // create the main Application window
    let window = adw::ApplicationWindow::builder()
        // The text on the titlebar
        .title(t!("app_name"))
        // link it to the application "app"
        .application(app)
        // Add the box called "window_box" to it
        // Application icon
        .icon_name(APP_ICON)
        // Minimum Size/Default
        .width_request(700)
        .height_request(500)
        // Hide window instead of destroy
        .hide_on_close(true)
        .deletable(false)
        // Startup
        .startup_id(APP_ID)
        // build the window
        .build();

    first_setup(&window);

    // show the window
    window.present()
}
