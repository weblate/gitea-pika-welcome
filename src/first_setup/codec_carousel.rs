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

const CODEC_INSTALL_PROG: &str = "
#! /bin/bash
set -e
sudo apt update -y && sudo apt install pika-codecs-meta -y
";

fn codec_install(
    log_loop_sender: async_channel::Sender<String>,
) -> Result<(), std::boxed::Box<dyn Error + Send + Sync>> {
    let (pipe_reader, pipe_writer) = os_pipe::pipe()?;
    let child = cmd!("bash", "-c", CODEC_INSTALL_PROG)
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

pub fn codec_carousel(
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

    let first_setup_codec_box = gtk::Box::builder()
        // that puts items vertically
        .orientation(Orientation::Vertical)
        .valign(gtk::Align::Center)
        .hexpand(true)
        .vexpand(true)
        .build();

    let first_setup_codec_box_text = adw::StatusPage::builder()
        .icon_name("media-tape")
        .title("Multi-media Codecs")
        .description("Would you like to install additional video playback and encoding/decoding packages?\n(strongly recommended)")
        .build();
    first_setup_codec_box_text.add_css_class("compact");

    let first_setup_codec_button = gtk::Button::builder()
        .label("Install Codecs")
        .sensitive(false)
        .build();

    first_setup_codec_button.add_css_class("suggested-action");
    first_setup_codec_button.add_css_class("pill");

    let first_setup_codec_skip_button = gtk::Button::builder()
        .label("Skip Codec Installation")
        .sensitive(true)
        .width_request(25)
        .build();

    let first_setup_codec_buttons_box = gtk::Box::builder()
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

    first_setup_codec_skip_button.add_css_class("pill");

    let codec_install_log_terminal_buffer = gtk::TextBuffer::builder().build();

    let codec_install_log_terminal = gtk::TextView::builder()
        .vexpand(true)
        .hexpand(true)
        .editable(false)
        .buffer(&codec_install_log_terminal_buffer)
        .build();

    let codec_install_log_terminal_scroll = gtk::ScrolledWindow::builder()
        .width_request(400)
        .height_request(200)
        .vexpand(true)
        .hexpand(true)
        .child(&codec_install_log_terminal)
        .build();

    let codec_install_dialog = adw::MessageDialog::builder()
        .transient_for(window)
        .hide_on_close(true)
        .extra_child(&codec_install_log_terminal_scroll)
        .width_request(400)
        .height_request(200)
        .heading("Codec installation Log")
        .build();
    codec_install_dialog.add_response("codec_install_dialog_ok", "Ok");

    first_setup_codec_buttons_box.append(&first_setup_codec_button);
    first_setup_codec_buttons_box.append(&first_setup_codec_skip_button);

    first_setup_codec_box.append(&first_setup_codec_box_text);
    first_setup_codec_box.append(&first_setup_codec_buttons_box);

    first_setup_carousel.append(&first_setup_codec_box);

    let internet_loop_context = MainContext::default();
    // The main loop executes the asynchronous block
    internet_loop_context.spawn_local(
        clone!(@strong internet_connected_status, @weak first_setup_codec_button => async move {
            while let Ok(_state) = internet_loop_receiver.recv().await {
                if *internet_connected_status.borrow_mut() == true {
                    first_setup_codec_button.set_sensitive(true);
                    first_setup_codec_button.set_label("Install Codecs");
                } else {
                    first_setup_codec_button.set_sensitive(false);
                    first_setup_codec_button.set_label("Disabled.. Network setup was skipped");
                }
            }
        }),
    );

    let log_loop_context = MainContext::default();
    // The main loop executes the asynchronous block
    log_loop_context.spawn_local(clone!(@weak codec_install_log_terminal_buffer, @weak codec_install_dialog => async move {
            while let Ok(state) = log_loop_receiver.recv().await {
                codec_install_log_terminal_buffer.insert(&mut codec_install_log_terminal_buffer.end_iter(), &("\n".to_string() + &state))
            }
    }));

    let log_status_loop_context = MainContext::default();
    // The main loop executes the asynchronous block
    log_status_loop_context.spawn_local(clone!(@weak codec_install_dialog, @weak first_setup_codec_button, @weak first_setup_codec_skip_button => async move {
            while let Ok(state) = log_status_loop_receiver.recv().await {
                if state == true {
                    codec_install_dialog.set_response_enabled("codec_install_dialog_ok", true);
                    codec_install_dialog.set_body("Codec installation Completed Successfully!");
                    first_setup_codec_button.remove_css_class("suggested-action");
                    first_setup_codec_skip_button.set_label("Next");
                    first_setup_codec_skip_button.add_css_class("suggested-action");
                } else {
                    first_setup_codec_skip_button.remove_css_class("suggested-action");
                    first_setup_codec_skip_button.set_label("Skip Codec Installation");
                    first_setup_codec_button.add_css_class("suggested-action");
                    codec_install_dialog.set_response_enabled("codec_install_dialog_ok", true);
                    codec_install_dialog.set_body("Codec installation Failed!\nPlease try again.");
                }
            }
    }));

    codec_install_log_terminal_buffer.connect_changed(clone!(@weak codec_install_log_terminal, @weak codec_install_log_terminal_buffer,@weak codec_install_log_terminal_scroll => move |_|{
       if codec_install_log_terminal_scroll.vadjustment().upper() - codec_install_log_terminal_scroll.vadjustment().value() > 100.0 {
            codec_install_log_terminal_scroll.vadjustment().set_value(codec_install_log_terminal_scroll.vadjustment().upper())
        }
    }));

    first_setup_codec_button.connect_clicked(clone!(@strong internet_connected_status, @weak codec_install_log_terminal,@weak codec_install_log_terminal_buffer, @weak codec_install_dialog,@weak first_setup_carousel => move |_| {
    codec_install_log_terminal_buffer.delete(&mut codec_install_log_terminal_buffer.bounds().0, &mut codec_install_log_terminal_buffer.bounds().1);
    codec_install_dialog.set_response_enabled("codec_install_dialog_ok", false);
    codec_install_dialog.set_body("");
    codec_install_dialog.present();
        // The long running operation runs now in a separate thread
    gio::spawn_blocking(clone!(@strong log_loop_sender, @strong log_status_loop_sender => move || {
        let command = codec_install(log_loop_sender);
        match command {
                Ok(_) => {
                    println!("Status: Codec install Successful");
                    log_status_loop_sender.send_blocking(true).expect("The channel needs to be open.");
                }
                Err(_) => {
                    println!("Status: Codec install Failed");
                    log_status_loop_sender.send_blocking(false).expect("The channel needs to be open.");
                }
        }
    }));
    }));

    first_setup_codec_skip_button.connect_clicked(clone!(@weak first_setup_carousel => move |_|{
        first_setup_carousel.scroll_to(&first_setup_carousel.nth_page(6), true);
    }));
}
