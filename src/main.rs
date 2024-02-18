// GTK crates
mod config;

use adw::prelude::*;
use adw::*;
use gdk::Display;
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)
use gtk::*;

use gettextrs::{gettext, LocaleCategory};
use users::*;
use config::{GETTEXT_PACKAGE, LOCALEDIR, APP_ID};

// application crates
mod build_ui;
use crate::build_ui::build_ui;
/// first setup crates
mod first_setup;

/// main function
fn main() {
    let application = adw::Application::new(
        Some(APP_ID),
        Default::default(),
    );
    application.connect_startup(|app| {
        // The CSS "magic" happens here.
        let provider = CssProvider::new();
        provider.load_from_string(include_str!("style.css"));
        // We give the CssProvided to the default screen so the CSS rules we added
        // can be applied to our window.
        gtk::style_context_add_provider_for_display(
            &Display::default().expect("Could not connect to a display."),
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        // Prepare i18n
        gettextrs::setlocale(LocaleCategory::LcAll, "");
        gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
        gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");
        // Fallback if no translation present
        if gettext("first_setup_initial_box_text_title") == "first_setup_initial_box_text_title" {
            println!("Warning: Current LANG is not supported, using fallback Locale.");
            gettextrs::setlocale(LocaleCategory::LcAll, "en_US.UTF8");
        }

        app.connect_activate(build_ui);
    });

    if get_current_username().unwrap() == "pikaos" {
        application.run();
    } else {
        println!("Error: This program can only be run via pikaos user");
        std::process::exit(1)
    }

}
