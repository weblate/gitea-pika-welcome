// GTK crates
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)
use gtk::prelude::*;
use gtk::*;
use adw::prelude::*;
use adw::*;
use glib::*;
use gdk::Display;

//use crate::check_internet_connection;
use std::process::Command;
use std::cell::RefCell;
use std::rc::Rc;
use std::borrow::Borrow as the_rc_borrow;
use std::env;
use gtk::gio::ffi::GAsyncReadyCallback;

pub fn internet_carousel(first_setup_carousel: &adw::Carousel, internet_connected: &Rc<RefCell<bool>>, window: &adw::ApplicationWindow) {

    let (internet_loop_sender, internet_loop_receiver) = async_channel::unbounded();
    let internet_loop_sender = internet_loop_sender.clone();
    // The long running operation runs now in a separate thread
    gio::spawn_blocking(move || {
        loop {
            //match check_internet_connection() {
            //    Ok(_) => {
            //        internet_loop_sender.send_blocking(true).expect("The channel needs to be open.");
            //    }
            //    Err(_) => {
            //        internet_loop_sender.send_blocking(false).expect("The channel needs to be open.");
            //    }
            //}
            let check_internet_connection_cli = Command::new("ping")
                .arg("ppa.pika-os.com")
                .arg("-c 1")
                .output()
                .expect("failed to execute process");
            if check_internet_connection_cli.status.success() {
                internet_loop_sender.send_blocking(true).expect("The channel needs to be open.");
            } else {
                internet_loop_sender.send_blocking(false).expect("The channel needs to be open.");
            }
        }
    });

    let first_setup_internet_box = gtk::Box::builder()
        // that puts items vertically
        .orientation(Orientation::Vertical)
        .hexpand(true)
        .vexpand(true)
        .margin_end(15)
        .margin_start(15)
        .margin_bottom(15)
        .margin_top(15)
        .build();

    let first_setup_internet_box_text = adw::StatusPage::builder()
        .icon_name("network-cellular-acquiring")
        .title("Internet")
        .description("Checking Internet Connection...")
        .hexpand(true)
        .vexpand(true)
        .valign(Align::Start)
        .build();
    first_setup_internet_box_text.add_css_class("compact");

    let internet_skip_button = gtk::Button::builder()
        .label("Skip")
        .halign(Align::Center)
        .sensitive(false)
        .build();

    internet_skip_button.add_css_class("destructive-action");
    internet_skip_button.add_css_class("pill");

    let internet_next_button = gtk::Button::builder()
        .label("Next")
        .halign(Align::Center)
        .sensitive(false)
        .build();

    internet_next_button.add_css_class("suggested-action");
    internet_next_button.add_css_class("pill");

    let internet_buttons_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(Align::Center)
        .valign(Align::End)
        .vexpand(true)
        .hexpand(true)
        .margin_end(15)
        .margin_start(15)
        .margin_bottom(15)
        .margin_top(15)
        .spacing(80)
        .build();

    let first_setup_internet_button_content_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let first_setup_internet_button_content_text = gtk::Label::builder()
        .label("Set up a network connection and a proxy/VPN.")
        .build();

    let first_setup_internet_button_content = adw::ButtonContent::builder()
        .label("Open Network Settings.")
        .icon_name("network-wired")
        .build();

    let first_setup_internet_button = gtk::Button::builder()
        .child(&first_setup_internet_button_content_box)
        .halign(Align::Center)
        .valign(Align::Center)
        .build();

    internet_buttons_box.append(&internet_skip_button);
    internet_buttons_box.append(&internet_next_button);

    first_setup_internet_button_content_box.append(&first_setup_internet_button_content);
    first_setup_internet_button_content_box.append(&first_setup_internet_button_content_text);

    first_setup_carousel.append(&first_setup_internet_box);

    first_setup_internet_box.append(&first_setup_internet_box_text);
    first_setup_internet_box.append(&first_setup_internet_button);
    first_setup_internet_box.append(&internet_buttons_box);

    let first_setup_internet_skip_dialog = adw::MessageDialog::builder()
        .heading("Skip Network Setup?")
        .body("Skipping Network Setup will make many of the next steps unavailable!\nThis is NOT recommended.")
        .transient_for(window)
        .hide_on_close(true)
        .build();

    first_setup_internet_skip_dialog.add_response("skip_cancel", "Return to Network Setup");
    first_setup_internet_skip_dialog.add_response("skip_confirm", "Just Skip!");
    first_setup_internet_skip_dialog.set_response_appearance("skip_confirm", adw::ResponseAppearance::Destructive);


    let internet_connected_status = internet_connected.clone();

    let internet_loop_context = MainContext::default();
    // The main loop executes the asynchronous block
    internet_loop_context.spawn_local(clone!(@weak internet_skip_button, @weak internet_next_button, @weak first_setup_internet_box_text => async move {
        while let Ok(state) = internet_loop_receiver.recv().await {
            if state == true {
                internet_skip_button.set_sensitive(false);
                internet_next_button.set_sensitive(true);
                first_setup_internet_box_text.set_description(Some("Device connected to Internet Successfully!"));
                first_setup_internet_box_text.set_icon_name(Some("network-cellular-signal-excellent"));
                *internet_connected_status.borrow_mut()=true;
            } else {
                internet_next_button.set_sensitive(false);
                internet_skip_button.set_sensitive(true);
                first_setup_internet_box_text.set_description(Some("No internet Connection!"));
                first_setup_internet_box_text.set_icon_name(Some("network-cellular-offline"));
                *internet_connected_status.borrow_mut()=false;
            }
        }
    }));

    first_setup_internet_button.connect_clicked(move |_| {
        let env_xdg_session_desktop = env::var("XDG_SESSION_DESKTOP").unwrap();
        if env_xdg_session_desktop.contains("gnome") || env_xdg_session_desktop.contains("ubuntu") {
            Command::new("gnome-control-center")
                .arg("network")
                .spawn()
                .expect("gnome-control-center failed to start");
        } else {
            Command::new("nm-connection-editor")
                .spawn()
                .expect("nm-connection-editor failed to start");
        }
    });

    internet_next_button.connect_clicked(clone!(@weak first_setup_carousel => move |_| {
        first_setup_carousel.scroll_to(&first_setup_carousel.nth_page(2), true);
    }));

    internet_skip_button.connect_clicked(clone!(@weak first_setup_carousel, @weak first_setup_internet_skip_dialog => move |_| {
        first_setup_internet_skip_dialog.choose(None::<&gio::Cancellable>, move |choice| {
            if choice == "skip_confirm" {
                first_setup_carousel.scroll_to(&first_setup_carousel.nth_page(2), true);
            }
        });
    }));
}