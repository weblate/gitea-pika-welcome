use adw::ffi::AdwApplicationWindow;
use adw::{gio, glib};
use adw::prelude::SettingsExt;
use adw::subclass::window;
use gtk::prelude::GtkWindowExt;
use glib::*;
use glib::prelude::*;
pub fn save_window_size(window: &adw::ApplicationWindow, glib_settings: &gio::Settings) {
    let size = window.default_size();

    let _ = glib_settings.set_int("window-width", size.0);
    let _ = glib_settings.set_int("window-height", size.1);
    let _ = glib_settings.set_boolean("is-maximized", window.is_maximized());
}
