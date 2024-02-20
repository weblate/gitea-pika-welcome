// GTK crates
use adw::prelude::*;
use adw::*;
use glib::*;
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)
use gtk::*;

//
use regex::Regex;
use std::cell::RefCell;
use std::rc::Rc;
use std::{thread, time};
use duct::cmd;

const USER_CREATE_PROG: &str = r###"
#! /bin/bash
USERNAME="$0"
PASSWORD="$1"
FULLNAME="$2"
HOSTNAME="$3"
adduser --quiet --disabled-password --shell /bin/bash --gecos "${FULLNAME}" "${USERNAME}"
echo "${USERNAME}":"${PASSWORD}" | chpasswd
usermod -a -G sudo "${USERNAME}"
mkdir -p /home/"${USERNAME}"
cp -rvf /etc/skel/.* /home/"${USERNAME}"/ || true
chown -R "${USERNAME}":"${USERNAME}" /home/"${USERNAME}"
usermod -a -G adm,cdrom,sudo,render,dip,video,plugdev,input,render,lpadmin "${USERNAME}"
rm -rf /etc/sddm.conf.d/zautologin.conf || true
hostnamectl set-hostname "${HOSTNAME}"
echo "127.0.0.1 ${HOSTNAME}" >> /etc/hosts
"###;

fn only_alphanumeric(input: &str) -> bool {
    return input.chars().all(|c| c.is_alphanumeric());
}

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

pub fn user_carousel(first_setup_carousel: &adw::Carousel) {
    let user_info_username_valid = Rc::new(RefCell::new(false));
    let user_info_full_name_valid = Rc::new(RefCell::new(false));
    let user_info_hostname_valid = Rc::new(RefCell::new(false));
    let user_info_passwords_valid = Rc::new(RefCell::new(false));

    let (user_loop_sender, user_loop_receiver) = async_channel::unbounded();
    let user_loop_sender = user_loop_sender.clone();
    // The long running operation runs now in a separate thread
    gio::spawn_blocking(move || loop {
        thread::sleep(time::Duration::from_secs(1));
        user_loop_sender
            .send_blocking(true)
            .expect("The channel needs to be open.");
    });

    let first_setup_user_box = gtk::Box::builder()
        // that puts items vertically
        .orientation(Orientation::Vertical)
        .hexpand(true)
        .vexpand(true)
        .margin_end(15)
        .margin_start(15)
        .margin_bottom(15)
        .margin_top(15)
        .build();

    let first_setup_user_box_text = adw::StatusPage::builder()
        .title(t!("first_setup_user_box_text_title"))
        .description(t!("first_setup_user_box_text_description"))
        .hexpand(true)
        .valign(Align::Start)
        .build();
    first_setup_user_box_text.add_css_class("compact");

    let user_info_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .valign(Align::Center)
        .vexpand(true)
        .hexpand(true)
        .build();

    let user_info_box_clamp = adw::Clamp::builder()
        .child(&user_info_box)
        .maximum_size(500)
        .build();

    let user_info_username = adw::EntryRow::builder()
        .hexpand(true)
        .title(t!("user_info_username_title"))
        .input_purpose(InputPurpose::Alpha)
        .input_hints(InputHints::LOWERCASE)
        .build();

    let user_info_full_name = adw::EntryRow::builder()
        .hexpand(true)
        .title(t!("user_info_full_name_title"))
        .input_purpose(InputPurpose::Name)
        .build();

    let user_info_password = adw::PasswordEntryRow::builder()
        .hexpand(true)
        .title(t!("user_info_password_title"))
        .build();

    let user_info_hostname = adw::EntryRow::builder()
        .hexpand(true)
        .title(t!("user_info_hostname_title"))
        .build();

    let user_info_password_verify = adw::PasswordEntryRow::builder()
        .hexpand(true)
        .title(t!("user_info_password_verify_title"))
        .build();

    let user_info_password_verify_revealer = gtk::Revealer::builder()
        .child(&user_info_password_verify)
        .reveal_child(false)
        .transition_type(RevealerTransitionType::SwingDown)
        .build();

    let user_info_avatar = adw::Avatar::builder()
        .show_initials(true)
        .size(128)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();

    let _user_info_avatar_full_name_binding = user_info_full_name
        .bind_property("text", &user_info_avatar, "text")
        .sync_create()
        .build();

    let user_info_listbox = gtk::ListBox::builder()
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();
    user_info_listbox.add_css_class("boxed-list");

    let error_label = gtk::Label::builder()
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .visible(false)
        .build();

    error_label.add_css_class("red-text");

    let user_next_button = gtk::Button::builder()
        .label(t!("internet_next_button_label"))
        .sensitive(false)
        .halign(Align::Center)
        .valign(Align::End)
        .vexpand(true)
        .hexpand(true)
        .build();

    user_next_button.add_css_class("suggested-action");
    user_next_button.add_css_class("pill");

    first_setup_carousel.append(&first_setup_user_box);

    user_info_listbox.append(&user_info_username);
    user_info_listbox.append(&user_info_full_name);
    user_info_listbox.append(&user_info_hostname);
    user_info_listbox.append(&user_info_password);
    user_info_listbox.append(&user_info_password_verify_revealer);

    user_info_box.append(&user_info_avatar);
    user_info_box.append(&user_info_listbox);

    first_setup_user_box.append(&first_setup_user_box_text);
    first_setup_user_box.append(&user_info_box_clamp);
    first_setup_user_box.append(&error_label);
    first_setup_user_box.append(&user_next_button);

    // The main loop executes the asynchronous block
    let user_loop_context = MainContext::default();
    user_loop_context.spawn_local(clone!(@strong user_info_username_valid, @strong user_info_full_name_valid,@strong user_info_hostname_valid, @strong user_info_passwords_valid, @weak user_next_button => async move {
        while let Ok(_state) = user_loop_receiver.recv().await {
            if *user_info_username_valid.borrow_mut() == true && *user_info_full_name_valid.borrow_mut() == true && *user_info_hostname_valid.borrow_mut() && *user_info_passwords_valid.borrow_mut() == true {
                user_next_button.set_sensitive(true);
            } else {
                user_next_button.set_sensitive(false);
            }
        }
    }));

    user_info_username.connect_changed(clone!(@strong user_info_username_valid, @weak user_info_username, @weak user_info_full_name, @weak error_label => move |_| {
        let user_info_username_string = user_info_username.text().to_string();

        user_info_full_name.set_text(&uppercase_first_letter(&user_info_username_string));

        if user_info_username_string.len() > 32 {
                user_info_username.set_text(&user_info_username_string[..32]);
                user_info_username.set_position(-1);
        }

        if Regex::new(r"[A-Z]").unwrap().is_match(&user_info_username_string) {
            user_info_username.set_text(&user_info_username_string.to_lowercase());
            user_info_username.set_position(-1);
        }

        let mut _username_is_root = false;
        let mut _username_is_pikaos = false;
        let mut _username_is_special = false;

        if user_info_username_string != "root" {
            _username_is_root=false;
        } else {
            error_label.set_label(&t!("error_label_is_root_label"));
            _username_is_root=true;
        }

        if user_info_username_string != "pikaos" {
            _username_is_pikaos=false;
        } else {
            error_label.set_label(&t!("error_label_is_pikaos_label"));
            _username_is_pikaos=true;
        }

        if only_alphanumeric(&user_info_username_string) {
            _username_is_special=false;
        } else {
            error_label.set_label(&t!("error_label_username_is_special_label"));
            _username_is_special=true;
        }

        if _username_is_root == false && _username_is_pikaos == false && _username_is_special == false {
            error_label.set_visible(false);
            if !user_info_username.text().is_empty() {
                *user_info_username_valid.borrow_mut()=true;
            }
        } else {
            *user_info_username_valid.borrow_mut()=false;
            error_label.set_visible(true);
        }
    }));

    user_info_full_name.connect_changed(clone!(@strong user_info_full_name_valid, @weak user_info_full_name, @weak error_label => move |_| {
        let user_info_full_name_string = user_info_full_name.text().to_string();

        if user_info_full_name_string.len() > 32 {
                user_info_full_name.set_text(&user_info_full_name_string[..32]);
                user_info_full_name.set_position(-1);
        }

        if user_info_full_name.text().is_empty() {
            *user_info_full_name_valid.borrow_mut()=false;
        } else {
            *user_info_full_name_valid.borrow_mut()=true;
        }
    }));

    user_info_hostname.connect_changed(clone!(@strong user_info_hostname_valid, @weak user_info_hostname, @weak user_info_full_name, @weak error_label => move |_| {
        let user_info_hostname_string = user_info_hostname.text().to_string();

        if user_info_hostname_string.len() > 32 {
                user_info_hostname.set_text(&user_info_hostname_string[..32]);
                user_info_hostname.set_position(-1);
        }

        let mut _hostname_is_special = false;

        if Regex::new(r"^[A-Za-z0-9\.]*$").unwrap().is_match(&user_info_hostname_string) {
            _hostname_is_special=false;
        } else {
            error_label.set_label(&t!("error_label_hostname_is_special_label"));
            _hostname_is_special=true;
        }

        if _hostname_is_special == false {
            error_label.set_visible(false);
            if !user_info_hostname.text().is_empty() {
                *user_info_hostname_valid.borrow_mut()=true;
            }
        } else {
            *user_info_hostname_valid.borrow_mut()=false;
            error_label.set_visible(true);
        }
    }));

    user_info_password.connect_changed(clone!(@strong user_info_passwords_valid,@weak user_info_password_verify_revealer, @weak user_info_password, @weak user_info_password_verify, @weak error_label => move |_| {
        if user_info_password.text().is_empty() {
            user_info_password_verify_revealer.set_reveal_child(false)
        } else {
            user_info_password_verify_revealer.set_reveal_child(true)
        }

        if user_info_password.text() == user_info_password_verify.text() {
            error_label.set_visible(false);
            *user_info_passwords_valid.borrow_mut()=true;
        } else {
            *user_info_passwords_valid.borrow_mut()=false;
        }
    }));

    user_info_password_verify.connect_changed(clone!(@strong user_info_passwords_valid, @weak user_info_password, @weak user_info_password_verify, @weak error_label => move |_| {
        if user_info_password.text() == user_info_password_verify.text() {
            error_label.set_visible(false);
            *user_info_passwords_valid.borrow_mut()=true;
        } else {
            error_label.set_visible(true);
            error_label.set_label(&t!("error_label_password_mismatch_label"));
            *user_info_passwords_valid.borrow_mut()=false;
        }
    }));

    user_next_button.connect_clicked(clone!(@weak first_setup_carousel, @weak user_info_username, @weak user_info_hostname, @weak user_info_password, @weak user_info_full_name, @weak user_info_hostname_valid => move |_| {
        let result = cmd!("/usr/lib/pika/pika-first-setup-gtk4/scripts/pika-sudo.sh", "bash", "-c", USER_CREATE_PROG, &user_info_username.text(), &user_info_password.text(), &user_info_full_name.text(), &user_info_hostname.text()).read();
        assert!(result.is_ok());
        first_setup_carousel.scroll_to(&first_setup_carousel.nth_page(3), true);
    }));
}
