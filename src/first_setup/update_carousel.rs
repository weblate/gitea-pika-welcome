use std::cell::RefCell;
use std::rc::Rc;
// GTK crates
use adw::ffi::ADW_TOOLBAR_FLAT;
use adw::prelude::*;
use adw::*;
use gdk::Display;
use glib::*;
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)
use gtk::prelude::*;
use gtk::*;
use vte::prelude::*;
use vte::*;

use gettextrs::gettext;

use std::{thread, time};

use std::{
    error::Error,
    io::Error as ErrorIO,
    io::{ErrorKind, Read, Write},
    process::{Command, Stdio},
};

use duct::cmd;
use os_pipe::*;
use std::io::prelude::*;
use std::io::BufReader;

const APT_UPDATE_PROG: &str = "
#! /bin/bash
set -e
sudo apt update -y && sudo apt full-upgrade -y
";

fn apt_update(
    log_loop_sender: async_channel::Sender<String>,
) -> Result<(), std::boxed::Box<dyn Error + Send + Sync>> {
    let (pipe_reader, pipe_writer) = os_pipe::pipe()?;
    let child = cmd!("bash", "-c", APT_UPDATE_PROG)
        .stderr_to_stdout()
        .stdout_file(pipe_writer)
        .start()?;
    for line in BufReader::new(pipe_reader).lines() {
        log_loop_sender
            .send_blocking(line?)
            .expect("Channel needs to be opened.")
    }
    child.wait()?;

    Ok(())
}

pub fn update_carousel(
    first_setup_carousel: &adw::Carousel,
    internet_connected: &Rc<RefCell<bool>>,
    window: &adw::ApplicationWindow,
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

    let (log_loop_sender, log_loop_receiver) = async_channel::unbounded();
    let log_loop_sender: async_channel::Sender<String> = log_loop_sender.clone();

    let (log_status_loop_sender, log_status_loop_receiver) = async_channel::unbounded();
    let log_status_loop_sender: async_channel::Sender<bool> = log_status_loop_sender.clone();

    let first_setup_update_box = gtk::Box::builder()
        // that puts items vertically
        .orientation(Orientation::Vertical)
        .valign(gtk::Align::Center)
        .hexpand(true)
        .vexpand(true)
        .build();

    let first_setup_update_box_text = adw::StatusPage::builder()
        .icon_name("software-update-available")
        .title(gettext("first_setup_update_box_text_title"))
        .description("We recommend updating your PikaOS install before proceeding\nWould you like to Update your system?")
        .build();
    first_setup_update_box_text.add_css_class("compact");

    let first_setup_update_button = gtk::Button::builder()
        .label(gettext("first_setup_update_button_label"))
        .sensitive(false)
        .build();

    first_setup_update_button.add_css_class("suggested-action");
    first_setup_update_button.add_css_class("pill");

    let first_setup_update_skip_button = gtk::Button::builder()
        .label(gettext("first_setup_update_skip_button_label"))
        .sensitive(true)
        .width_request(25)
        .build();

    let first_setup_update_buttons_box = gtk::Box::builder()
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

    first_setup_update_skip_button.add_css_class("pill");

    let system_update_log_terminal_buffer = gtk::TextBuffer::builder().build();

    let system_update_log_terminal = gtk::TextView::builder()
        .vexpand(true)
        .hexpand(true)
        .editable(false)
        .buffer(&system_update_log_terminal_buffer)
        .build();

    let system_update_log_terminal_scroll = gtk::ScrolledWindow::builder()
        .width_request(400)
        .height_request(200)
        .vexpand(true)
        .hexpand(true)
        .child(&system_update_log_terminal)
        .build();

    let system_update_dialog = adw::MessageDialog::builder()
        .transient_for(window)
        .hide_on_close(true)
        .extra_child(&system_update_log_terminal_scroll)
        .width_request(400)
        .height_request(200)
        .heading(gettext("system_update_dialog_heading"))
        .build();
    system_update_dialog.add_response("system_update_dialog_ok", &gettext("system_update_dialog_ok_label"));

    first_setup_update_buttons_box.append(&first_setup_update_button);
    first_setup_update_buttons_box.append(&first_setup_update_skip_button);

    first_setup_update_box.append(&first_setup_update_box_text);
    first_setup_update_box.append(&first_setup_update_buttons_box);

    first_setup_carousel.append(&first_setup_update_box);

    let internet_loop_context = MainContext::default();
    // The main loop executes the asynchronous block
    internet_loop_context.spawn_local(
        clone!(@strong internet_connected_status, @weak first_setup_update_button => async move {
            while let Ok(_state) = internet_loop_receiver.recv().await {
                if *internet_connected_status.borrow_mut() == true {
                    first_setup_update_button.set_sensitive(true);
                    first_setup_update_button.set_label(&gettext("first_setup_update_button_label"));
                } else {
                    first_setup_update_button.set_sensitive(false);
                    first_setup_update_button.set_label(&gettext("internet_network_disabled"));
                }
            }
        }),
    );

    let log_loop_context = MainContext::default();
    // The main loop executes the asynchronous block
    log_loop_context.spawn_local(clone!(@weak system_update_log_terminal_buffer, @weak system_update_dialog => async move {
            while let Ok(state) = log_loop_receiver.recv().await {
                system_update_log_terminal_buffer.insert(&mut system_update_log_terminal_buffer.end_iter(), &("\n".to_string() + &state))
            }
    }));

    let log_status_loop_context = MainContext::default();
    // The main loop executes the asynchronous block
    log_status_loop_context.spawn_local(clone!(@weak system_update_dialog, @weak first_setup_update_button, @weak first_setup_update_skip_button => async move {
            while let Ok(state) = log_status_loop_receiver.recv().await {
                if state == true {
                    system_update_dialog.set_response_enabled("system_update_dialog_ok", true);
                    system_update_dialog.set_body(&gettext("system_update_dialog_success_true"));
                    first_setup_update_button.remove_css_class("suggested-action");
                    first_setup_update_skip_button.set_label(&gettext("internet_next_button_label"));
                    first_setup_update_skip_button.add_css_class("suggested-action");
                } else {
                    first_setup_update_skip_button.remove_css_class("suggested-action");
                    first_setup_update_skip_button.set_label(&gettext("internet_skip_button_label"));
                    first_setup_update_button.add_css_class("suggested-action");
                    system_update_dialog.set_response_enabled("system_update_dialog_ok", true);
                    system_update_dialog.set_body(&gettext("system_update_dialog_success_false"));
                }
            }
    }));

    system_update_log_terminal_buffer.connect_changed(clone!(@weak system_update_log_terminal, @weak system_update_log_terminal_buffer,@weak system_update_log_terminal_scroll => move |_|{
       if system_update_log_terminal_scroll.vadjustment().upper() - system_update_log_terminal_scroll.vadjustment().value() > 100.0 {
            system_update_log_terminal_scroll.vadjustment().set_value(system_update_log_terminal_scroll.vadjustment().upper())
        }
    }));

    first_setup_update_button.connect_clicked(clone!(@strong internet_connected_status, @weak system_update_log_terminal,@weak system_update_log_terminal_buffer, @weak system_update_dialog,@weak first_setup_carousel => move |_| {
    system_update_log_terminal_buffer.delete(&mut system_update_log_terminal_buffer.bounds().0, &mut system_update_log_terminal_buffer.bounds().1);
    system_update_dialog.set_response_enabled("system_update_dialog_ok", false);
    system_update_dialog.set_body("");
    system_update_dialog.present();
        // The long running operation runs now in a separate thread
    gio::spawn_blocking(clone!(@strong log_loop_sender, @strong log_status_loop_sender => move || {
        let command = apt_update(log_loop_sender);
        match command {
                Ok(_) => {
                    println!("Status: Apt System Upgrade Successful");
                    log_status_loop_sender.send_blocking(true).expect("The channel needs to be open.");
                }
                Err(_) => {
                    println!("Status: Apt System Failed");
                    log_status_loop_sender.send_blocking(false).expect("The channel needs to be open.");
                }
        }
    }));
    }));

    first_setup_update_skip_button.connect_clicked(clone!(@weak first_setup_carousel => move |_|{
        first_setup_carousel.scroll_to(&first_setup_carousel.nth_page(4), true);
    }));
}
