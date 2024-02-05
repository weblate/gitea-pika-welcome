use std::cell::RefCell;
use std::process::Command;
use std::rc::Rc;
// GTK crates
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)
use gtk::prelude::*;
use gtk::*;
use adw::prelude::*;
use adw::*;
use adw::ffi::ADW_TOOLBAR_FLAT;
use glib::*;
use gdk::Display;

use std::{thread, time};

pub fn update_carousel(first_setup_carousel: &adw::Carousel, internet_connected: &Rc<RefCell<bool>>) {

    let internet_connected_status = internet_connected.clone();

    let (internet_loop_sender, internet_loop_receiver) = async_channel::unbounded();
    let internet_loop_sender = internet_loop_sender.clone();
    // The long running operation runs now in a separate thread
    gio::spawn_blocking(move || {
        loop {
            thread::sleep(time::Duration::from_secs(1));
            internet_loop_sender.send_blocking(true).expect("The channel needs to be open.");
        }
    });

    let first_setup_update_box = gtk::Box::builder()
        // that puts items vertically
        .orientation(Orientation::Vertical)
        .vexpand(true)
        .valign(Align::Center)
        .hexpand(true)
        .vexpand(true)
        .build();

    let first_setup_update_box_text = adw::StatusPage::builder()
        .icon_name("software-update-available")
        .title("System Updates")
        .description("We recommend updating your PikaOS install before proceeding\nWould you like to Update your system?")
        .build();
    first_setup_update_box_text.add_css_class("compact");

    let first_setup_update_button = gtk::Button::builder()
        .label("Update")
        .sensitive(false)
        .halign(Align::Center)
        .build();

    first_setup_update_button.add_css_class("suggested-action");
    first_setup_update_button.add_css_class("pill");

    first_setup_update_box.append(&first_setup_update_box_text);
    first_setup_update_box.append(&first_setup_update_button);

    first_setup_carousel.append(&first_setup_update_box);

    let internet_loop_context = MainContext::default();
    // The main loop executes the asynchronous block
    internet_loop_context.spawn_local(clone!(@strong internet_connected_status, @weak first_setup_update_button => async move {
        while let Ok(_state) = internet_loop_receiver.recv().await {
            if *internet_connected_status.borrow_mut() == true {
                first_setup_update_button.set_sensitive(true);
                first_setup_update_button.set_label("Update");
            } else {
                first_setup_update_button.set_sensitive(false);
                first_setup_update_button.set_label("Disabled.. Network setup was skipped");
            }
        }
    }));

    first_setup_update_button.connect_clicked(clone!(@strong internet_connected_status, @weak first_setup_carousel => move |_| {
        first_setup_carousel.scroll_to(&first_setup_carousel.nth_page(4), true);
    }));
}