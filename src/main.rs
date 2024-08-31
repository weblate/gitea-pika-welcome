// GTK crates
mod config;

use adw::prelude::*;
use adw::*;
use gdk::Display;
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)
use gtk::*;
use single_instance::SingleInstance;
use std::env;

use config::APP_ID;
use std::boxed::Box;

// application crates
mod build_ui;
use crate::build_ui::build_ui;
mod save_window_size;
/// first setup crates
mod welcome_content_page;

// Init translations for current crate.
use rust_i18n::Backend;
use std::collections::HashMap;
use std::fs;

pub struct I18nBackend {
    trs: HashMap<String, HashMap<String, String>>,
}
impl I18nBackend {
    fn new() -> Self {
        let mut trs = HashMap::new();
        let locales_dir = fs::read_dir("/usr/lib/pika/pika-welcome/locales").expect("No translation files found");
        for locale_file in locales_dir {
            let locale_file_path = locale_file.expect("couldn't change dir entry to path").path();
            let locale = String::from(locale_file_path.file_name().unwrap().to_str().unwrap().trim_end_matches(".json"));
            let locale_data = fs::read_to_string(locale_file_path).expect(format!("invalid json for {}", locale).as_str());
            let locale_json = serde_json::from_str::<HashMap<String, String>>(&locale_data).unwrap();
            trs.insert(locale.to_string(), locale_json);
        }

        return Self {
            trs
        };
    }
}

impl Backend for I18nBackend {
    fn available_locales(&self) -> Vec<&str> {
        return self.trs.keys().map(|k| k.as_str()).collect();
    }

    fn translate(&self, locale: &str, key: &str) -> Option<&str> {
        return self.trs.get(locale)?.get(key).map(|k| k.as_str());
    }
}

#[macro_use]
extern crate rust_i18n;
i18n!(fallback = "en_US", backend = I18nBackend::new());

/// main function
fn main() {
    let current_locale = match env::var_os("LANG") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$LANG is not set"),
    };
    rust_i18n::set_locale(current_locale.strip_suffix(".UTF-8").unwrap());
    let application = adw::Application::new(Some(APP_ID), Default::default());
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
        app.connect_activate(build_ui);
    });

    let instance = SingleInstance::new(APP_ID).unwrap();
    assert!(instance.is_single());

    application.run();
}
