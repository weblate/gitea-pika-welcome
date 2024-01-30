// GTK crates
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)
use gtk::prelude::*;
use gtk::*;
use adw::prelude::*;
use adw::*;
use glib::*;
use gdk::Display;

// application crates
use crate::save_window_size;

pub fn build_ui(app: &adw::Application) {

    // setup glib
    gtk::glib::set_prgname(Some("Cosmo libAdwaita in RS Template"));
    glib::set_application_name("Cosmo libAdwaita in RS Template");
    let glib_settings = gio::Settings::new("com.github.adw-rs-test.cosmo");

    // Create a label called "_warning_label"
    let _warning_label = Label::builder()
        // Label Text
        .label("DO NOT LISTEN TO THE BUTTON!")
        // Add Space to the top
        .margin_top(12)
        // Add Space to the buttom
        .margin_bottom(12)
        // Add Space to the left
        .margin_start(12)
        // Add Space to the right
        .margin_end(12)
        // build the button
        .build();

    // Create a button called "_click_me_button"
    let _click_me_button = Button::builder()
        // Button Label
        .label("Click ME")
        // Add Space to the top
        .margin_top(12)
        // Add Space to the buttom
        .margin_bottom(12)
        // Add Space to the left
        .margin_start(12)
        // Add Space to the right
        .margin_end(12)
        // build the button
        .build();

    // Create A box
    let _main_box = gtk::Box::builder()
        // that puts items vertically
        .orientation(Orientation::Vertical)
        .build();

    // Add adwaita title box
    let window_title_bar = gtk::HeaderBar::builder()
        .show_title_buttons(true)
        .build();

    _main_box.append(&window_title_bar);

    // Add the "_warning_label" to "_main_box"
    _main_box.append(&_warning_label);
    // Add the "_click_me_button" to "_main_box"
    _main_box.append(&_click_me_button);




    // create the main Application window
    let window = adw::ApplicationWindow::builder()
        // The text on the titlebar
        .title("Libadwaita gtk4 example window in rust")
        // link it to the application "app"
        .application(app)
        // Add the box called "_main_box" to it
        .content(&_main_box)
        // Application icon
        .icon_name("org.cosmicfusion.example")
        // Get current size from glib
        .default_width(glib_settings.int("window-width"))
        .default_height(glib_settings.int("window-height"))
        // Minimum Size/Default
        .width_request(700)
        .height_request(500)
        // Hide window instead of destroy
        .hide_on_close(true)
        // Startup
        .startup_id("org.cosmicfusion.example")
        // build the window
        .build();

    // glib maximization
    if glib_settings.boolean("is-maximized") == true {
        window.maximize()
    }

    // Connects the clicking of  "_click_me_button" to the external function "print_why" and idk why but everyone tells me to be "move |_| " before the external function
    /// and instead of () we put an aurgment for the target label with & before it so it's"
    /// print_why() -> print_why(&_warning_label)
    _click_me_button.connect_clicked(move |_| print_why(&_warning_label));

    // Connect the hiding of window to the save_window_size function and window destruction
    window.connect_hide(clone!(@weak window => move |_| save_window_size(&window, &glib_settings)));
    window.connect_hide(clone!(@weak window => move |_| window.destroy()));

    // show the window
    window.present()
}


// an external function to be called via "_click_me_button.connect_clicked"
fn print_why(label: &Label) {
    // takes the aurgument from "_click_me_button.connect_clicked" which should be a label amd sets its text to "Why would you :("
    label.set_text("Why would you :(");
    label.add_css_class("bigREDTEXT");
}
