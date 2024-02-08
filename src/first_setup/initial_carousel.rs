// GTK crates
use adw::prelude::*;
use adw::*;
use gdk::Display;
use glib::*;
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)
use gtk::prelude::*;
use gtk::*;

pub fn initial_carousel(first_setup_carousel: &adw::Carousel) {
    let first_setup_initial_box = gtk::Box::builder()
        // that puts items vertically
        .orientation(Orientation::Vertical)
        .vexpand(true)
        .valign(Align::Center)
        .hexpand(true)
        .vexpand(true)
        .build();

    let first_setup_initial_box_text = adw::StatusPage::builder()
        .icon_name("debian-swirl")
        .title("Welcome")
        .description("This wizard will help you finish your PikaOS installation.")
        .build();
    first_setup_initial_box_text.add_css_class("compact");

    let first_setup_start_button = gtk::Button::builder()
        .label("Let's Start")
        .halign(Align::Center)
        .build();

    first_setup_start_button.add_css_class("suggested-action");
    first_setup_start_button.add_css_class("pill");

    first_setup_initial_box.append(&first_setup_initial_box_text);
    first_setup_initial_box.append(&first_setup_start_button);

    first_setup_carousel.append(&first_setup_initial_box);

    first_setup_start_button.connect_clicked(clone!(@weak first_setup_carousel => move |_| {
        first_setup_carousel.scroll_to(&first_setup_carousel.nth_page(1), true)
    }));
}
