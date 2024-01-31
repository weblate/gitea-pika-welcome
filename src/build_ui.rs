// GTK crates
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)
use gtk::prelude::*;
use gtk::*;
use adw::prelude::*;
use adw::*;
use adw::ffi::{ADW_TOOLBAR_FLAT, AdwToolbarStyle};
use glib::*;
use gdk::Display;

// application crates
use crate::save_window_size;
/// first setup crates
use crate::first_setup::first_setup::first_setup;

pub fn build_ui(app: &adw::Application) {

    // setup glib
    gtk::glib::set_prgname(Some("PikaOS First Setup"));
    glib::set_application_name("PikaOS First Setup");
    let glib_settings = gio::Settings::new("com.github.pikaos-linux.pikafirstsetup");

    // create the main Application window
    let window = adw::ApplicationWindow::builder()
        // The text on the titlebar
        .title("PikaOS First Setup")
        // link it to the application "app"
        .application(app)
        // Add the box called "window_box" to it
        // Application icon
        .icon_name("com.github.pikaos-linux.pikafirstsetup")
        // Get current size from glib
        .default_width(glib_settings.int("window-width"))
        .default_height(glib_settings.int("window-height"))
        // Minimum Size/Default
        .width_request(700)
        .height_request(500)
        // Hide window instead of destroy
        .hide_on_close(true)
        .deletable(false)
        // Startup
        .startup_id("com.github.pikaos-linux.pikafirstsetup")
        // build the window
        .build();

    // glib maximization
    if glib_settings.boolean("is-maximized") == true {
        window.maximize()
    }

    // Connect the hiding of window to the save_window_size function and window destruction
    window.connect_hide(clone!(@weak window => move |_| save_window_size(&window, &glib_settings)));
    window.connect_hide(clone!(@weak window => move |_| window.destroy()));

    first_setup(&window);

    // show the window
    window.present()
}