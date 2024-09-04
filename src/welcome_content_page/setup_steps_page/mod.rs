// GTK crates
use adw::prelude::*;
use adw::*;
use duct::cmd;
use glib::*;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug, Eq, Hash, Clone, Ord, PartialOrd, Deserialize)]
struct setup_steps_entry {
    id: i32,
    title: String,
    subtitle: String,
    icon: String,
    button: String,
    command: String,
}

pub fn setup_steps_page(
    setup_steps_content_page_stack: &gtk::Stack,
    window: &adw::ApplicationWindow,
) {
    let setup_steps_page_box = gtk::Box::builder().vexpand(true).hexpand(true).build();

    let setup_steps_page_listbox = gtk::ListBox::builder()
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .vexpand(true)
        .hexpand(true)
        .build();
    setup_steps_page_listbox.add_css_class("boxed-list");

    let setup_steps_page_scroll = gtk::ScrolledWindow::builder()
        // that puts items vertically
        .hexpand(true)
        .vexpand(true)
        .child(&setup_steps_page_box)
        .propagate_natural_width(true)
        .propagate_natural_height(true)
        .min_content_width(520)
        .build();

    let mut json_array: Vec<setup_steps_entry> = Vec::new();
    let json_path = "/usr/share/pika-welcome/config/setup_steps.json";
    let json_data = fs::read_to_string(json_path).expect("Unable to read json");
    let json_data: serde_json::Value =
        serde_json::from_str(&json_data).expect("JSON format invalid");
    if let serde_json::Value::Array(setup_steps) = &json_data["setup_steps"] {
        for setup_steps_entry in setup_steps {
            let setup_steps_entry_struct: setup_steps_entry =
                serde_json::from_value(setup_steps_entry.clone()).unwrap();
            json_array.push(setup_steps_entry_struct);
        }
    }

    let entry_buttons_size_group = gtk::SizeGroup::new(gtk::SizeGroupMode::Both);

    for setup_steps_entry in json_array {
        let (entry_command_status_loop_sender, entry_command_status_loop_receiver) =
            async_channel::unbounded();
        let entry_command_status_loop_sender: async_channel::Sender<bool> =
            entry_command_status_loop_sender.clone();

        let entry_title = setup_steps_entry.title;
        let entry_subtitle = setup_steps_entry.subtitle;
        let entry_icon = setup_steps_entry.icon;
        let entry_button = setup_steps_entry.button;
        let entry_command = setup_steps_entry.command;
        let entry_row = adw::ActionRow::builder()
            .title(t!(&entry_title))
            .subtitle(t!(&entry_subtitle))
            .vexpand(true)
            .hexpand(true)
            .build();
        let entry_row_icon = gtk::Image::builder()
            .icon_name(entry_icon)
            .pixel_size(80)
            .vexpand(true)
            .valign(gtk::Align::Center)
            .build();
        let entry_row_button = gtk::Button::builder()
            .label(t!(&entry_button))
            .vexpand(true)
            .valign(gtk::Align::Center)
            .build();
        entry_buttons_size_group.add_widget(&entry_row_button);
        entry_row.add_prefix(&entry_row_icon);
        entry_row.add_suffix(&entry_row_button);

        entry_row_button.connect_clicked(clone!(@strong entry_command, @weak window => move |_| {
                gio::spawn_blocking(clone!(@strong entry_command_status_loop_sender, @strong entry_command => move || {
                            if Path::new("/tmp/pika-welcome-exec.sh").exists() {
                            fs::remove_file("/tmp/pika-welcome-exec.sh").expect("Bad permissions on /tmp/pika-installer-gtk4-target-manual.txt");
                            }
                            fs::write("/tmp/pika-welcome-exec.sh", "#! /bin/bash\nset -e\n".to_owned() + &entry_command).expect("Unable to write file");
                            let _ = cmd!("chmod", "+x", "/tmp/pika-welcome-exec.sh").read();
                            let command = cmd!("/tmp/pika-welcome-exec.sh").run();
                            if command.is_err() {
                                entry_command_status_loop_sender.send_blocking(false).expect("The channel needs to be open.");
                            } else {
                                entry_command_status_loop_sender.send_blocking(true).expect("The channel needs to be open.");
                            }
                }));
        }));

        let cmd_err_dialog = adw::MessageDialog::builder()
            .body(t!("cmd_err_dialog_body"))
            .heading(t!("cmd_err_dialog_heading"))
            .transient_for(window)
            .build();
        cmd_err_dialog.add_response(
            "cmd_err_dialog_ok",
            &t!("cmd_err_dialog_ok_label").to_string(),
        );

        let entry_command_status_loop_context = MainContext::default();
        // The main loop executes the asynchronous block
        entry_command_status_loop_context.spawn_local(
            clone!(@weak cmd_err_dialog, @strong entry_command_status_loop_receiver => async move {
                while let Ok(state) = entry_command_status_loop_receiver.recv().await {
                    if state == false {
                        cmd_err_dialog.present();
                    }
                }
            }),
        );
        setup_steps_page_listbox.append(&entry_row)
    }

    setup_steps_page_box.append(&setup_steps_page_listbox);

    setup_steps_content_page_stack.add_titled(
        &setup_steps_page_scroll,
        Some("setup_steps_page"),
        &t!("setup_steps_page_title").to_string(),
    );
}
