// GTK crates
use adw::prelude::*;
use adw::*;
use glib::*;
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)



//
use std::cell::RefCell;
use std::process::Command;
use std::rc::Rc;
use adw::glib::gobject_ffi::GValue;
use gtk::Orientation::Horizontal;

// carousel crates
mod update_carousel;

use update_carousel::*;
use crate::config::{APP_GITHUB, APP_ICON, VERSION};

pub fn welcome_content_page(window: &adw::ApplicationWindow, content_box: &gtk::Box) {
    let internet_connected = Rc::new(RefCell::new(false));

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
                internet_loop_sender
                    .send_blocking(true)
                    .expect("The channel needs to be open.");
            } else {
                internet_loop_sender
                    .send_blocking(false)
                    .expect("The channel needs to be open.");
            }
        }
    });

    let window_title_bar = gtk::HeaderBar::builder().show_title_buttons(true).build();

    let credits_button = gtk::Button::builder()
        .icon_name("dialog-information-symbolic")
        .build();

    let credits_window = adw::AboutWindow::builder()
        .application_icon(APP_ICON)
        .application_name(t!("app_name"))
        .transient_for(window)
        .version(VERSION)
        .hide_on_close(true)
        .developer_name(t!("app_dev"))
        .issue_url(APP_GITHUB.to_owned() + "/issues")
        .build();

    content_box.append(&window_title_bar);

    let welcome_content_page_stack = gtk::Stack::builder()
        .vexpand(true)
        .hexpand(true)
        .build();

    let welcome_content_page_stack_sidebar = gtk::StackSidebar::builder()
        .vexpand(true)
        .hexpand(true)
        .stack(&welcome_content_page_stack)
        .build();

    let welcome_content_page_split_view = adw::OverlaySplitView::builder()
        .vexpand(true)
        .hexpand(true)
        .content(&welcome_content_page_stack)
        .sidebar(&welcome_content_page_stack_sidebar)
        .max_sidebar_width(300.0)
        .min_sidebar_width(300.0)
        .enable_hide_gesture(true)
        .enable_show_gesture(true)
        .build();

    let sidebar_toggle_button = gtk::ToggleButton::builder()
        .icon_name("view-right-pane-symbolic")
        .build();

    let _sidebar_toggle_button_binding = welcome_content_page_split_view
        .bind_property("show_sidebar", &sidebar_toggle_button, "active")
        .sync_create()
        .bidirectional()
        .build();

    let welcome_content_page_split_view_breakpoint = adw::Breakpoint::new(BreakpointCondition::new_length(BreakpointConditionLengthType::MaxWidth, 400.0, LengthUnit::Px));
    welcome_content_page_split_view_breakpoint.add_setter(&welcome_content_page_split_view, "show_sidebar", &false.to_value());

    window.add_breakpoint(welcome_content_page_split_view_breakpoint);

    window_title_bar.pack_end(&credits_button);
    window_title_bar.pack_start(&sidebar_toggle_button);
    content_box.append(&welcome_content_page_split_view);

    credits_button
        .connect_clicked(clone!(@weak credits_button => move |_| credits_window.present()));

    let internet_connected_status = internet_connected.clone();

    let internet_loop_context = MainContext::default();
    // The main loop executes the asynchronous block
    internet_loop_context.spawn_local(clone!(@weak window => async move {
        while let Ok(state) = internet_loop_receiver.recv().await {
            if state == true {
                *internet_connected_status.borrow_mut()=true;
            } else {
                *internet_connected_status.borrow_mut()=false;
            }
        }
    }));
}
