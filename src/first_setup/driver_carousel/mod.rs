use std::cell::RefCell;
use std::rc::Rc;
// GTK crates
use adw::prelude::*;
use adw::*;
use glib::*;
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)
use gtk::*;



use std::{thread, time};

use std::{
    process::{Command},
};

pub fn driver_carousel(
    first_setup_carousel: &adw::Carousel,
    internet_connected: &Rc<RefCell<bool>>,
) {
    let internet_connected_status = internet_connected.clone();

    let (internet_loop_sender, internet_loop_receiver) = async_channel::unbounded();
    let internet_loop_sender = internet_loop_sender.clone();
    // The long running operation runs now in a separate thread
    gio::spawn_blocking(move || loop {
        thread::sleep(time::Duration::from_secs(1));
        internet_loop_sender
            .send_blocking(true)
            .expect("The channel needs to be open.");
    });

    let first_setup_driver_box = gtk::Box::builder()
        // that puts items vertically
        .orientation(Orientation::Vertical)
        .valign(gtk::Align::Center)
        .hexpand(true)
        .vexpand(true)
        .build();

    let first_setup_driver_box_text = adw::StatusPage::builder()
        .icon_name("audio-card")
        .title(t!("first_setup_driver_box_text_title"))
        .description(t!("first_setup_driver_box_text_description"))
        .build();
    first_setup_driver_box_text.add_css_class("compact");

    let first_setup_driver_button = gtk::Button::builder()
        .label(t!("first_setup_driver_button_label"))
        .sensitive(false)
        .build();

    first_setup_driver_button.add_css_class("suggested-action");
    first_setup_driver_button.add_css_class("pill");

    let first_setup_driver_skip_button = gtk::Button::builder()
        .label(t!("first_setup_driver_skip_button_label"))
        .sensitive(true)
        .width_request(25)
        .build();

    let first_setup_driver_buttons_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::End)
        .vexpand(true)
        .hexpand(true)
        .margin_end(15)
        .margin_start(15)
        .margin_bottom(15)
        .margin_top(15)
        .spacing(80)
        .build();

    first_setup_driver_skip_button.add_css_class("pill");

    first_setup_driver_buttons_box.append(&first_setup_driver_button);
    first_setup_driver_buttons_box.append(&first_setup_driver_skip_button);

    first_setup_driver_box.append(&first_setup_driver_box_text);
    first_setup_driver_box.append(&first_setup_driver_buttons_box);

    first_setup_carousel.append(&first_setup_driver_box);

    let internet_loop_context = MainContext::default();
    // The main loop executes the asynchronous block
    internet_loop_context.spawn_local(
        clone!(@strong internet_connected_status, @weak first_setup_driver_button => async move {
            while let Ok(_state) = internet_loop_receiver.recv().await {
                if *internet_connected_status.borrow_mut() == true {
                    first_setup_driver_button.set_sensitive(true);
                    first_setup_driver_button.set_label(&t!("first_setup_driver_button_label"));
                } else {
                    first_setup_driver_button.set_sensitive(false);
                    first_setup_driver_button.set_label(&t!("internet_network_disabled"));
                }
            }
        }),
    );

    first_setup_driver_button.connect_clicked(clone!(@weak first_setup_carousel, @weak first_setup_driver_button, @weak first_setup_driver_skip_button => move |_| {
        Command::new("pika-drivers")
            .spawn()
            .expect("pika-drivers failed to start");
        first_setup_driver_button.remove_css_class("suggested-action");
        first_setup_driver_skip_button.set_label(&t!("internet_next_button_label"));
        first_setup_driver_skip_button.add_css_class("suggested-action");
    }));

    first_setup_driver_skip_button.connect_clicked(clone!(@weak first_setup_carousel => move |_|{
        first_setup_carousel.scroll_to(&first_setup_carousel.nth_page(5), true);
    }));
}
