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
    let welcome_page_box = gtk::Box::builder()
        // that puts items vertically
        .orientation(Orientation::Vertical)
        .valign(gtk::Align::Center)
        .hexpand(true)
        .vexpand(true)
        .build();

    let welcome_page_text = adw::StatusPage::builder()
        .icon_name(DISTRO_ICON)
        .title(t!("welcome_page_text_title"))
        .description(t!("welcome_page_text_description"))
        .build();
    welcome_page_text.add_css_class("compact");

    welcome_page_box.append(&welcome_page_text);

    welcome_content_page_stack.add_titled(&welcome_page_box, Some("welcome_page"), &t!("welcome_page_title").to_string());
}