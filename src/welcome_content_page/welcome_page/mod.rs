// GTK crates
use std::rc::Rc;
use std::cell::RefCell;
use adw::prelude::*;
use adw::*;
use adw::gio::ffi::GReallocFunc;
use glib::*;
use gtk::Orientation;
use crate::config::DISTRO_ICON;

pub fn welcome_page(
    welcome_content_page_stack: &gtk::Stack,
    internet_connected: &Rc<RefCell<bool>>
) {
    let welcome_page_text = adw::StatusPage::builder()
        .icon_name(DISTRO_ICON)
        .title(t!("welcome_page_text_title"))
        .description(t!("welcome_page_text_description"))
        .build();
    welcome_page_text.add_css_class("compact");

    let welcome_page_scroll = gtk::ScrolledWindow::builder()
        // that puts items vertically
        .valign(gtk::Align::Center)
        .hexpand(true)
        .vexpand(true)
        .child(&welcome_page_text)
        .propagate_natural_width(true)
        .propagate_natural_height(true)
        .build();

    welcome_content_page_stack.add_titled(&welcome_page_scroll, Some("welcome_page"), &t!("welcome_page_title").to_string());
}