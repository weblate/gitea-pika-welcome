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

//
use crate::connection_check::check_internet_connection;
use std::cell::RefCell;
use std::rc::Rc;

// carousel crates
use crate::first_setup::initial_carousel::initial_carousel;
use crate::first_setup::internet_carousel::internet_carousel;
use crate::first_setup::update_carousel::update_carousel;
use crate::first_setup::user_carousel::user_carousel;
use crate::first_setup::driver_carousel::driver_carousel;
use crate::first_setup::codec_carousel::codec_carousel;
use crate::first_setup::gameutils_carousel::gameutils_carousel;

pub fn first_setup(window: &adw::ApplicationWindow) {
    let first_setup_carousel = adw::Carousel::builder()
        .allow_long_swipes(false)
        .allow_mouse_drag(false)
        .allow_scroll_wheel(false)
        .interactive(false)
        .vexpand(true)
        .hexpand(true)
        .build();

    let first_setup_carousel_indicator = adw::CarouselIndicatorDots::builder()
        .carousel(&first_setup_carousel)
        .build();

    let first_setup_window_headerbar_back_button = gtk::Button::builder().label("Back").build();

    let first_setup_window_headerbar = adw::HeaderBar::builder()
        .show_start_title_buttons(true)
        .title_widget(&first_setup_carousel_indicator)
        .build();

    let first_setup_window_toolbarview = adw::ToolbarView::builder()
        .top_bar_style(ToolbarStyle::Flat)
        .content(&first_setup_carousel)
        .build();

    let internet_connected = Rc::new(RefCell::new(false));

    first_setup_window_headerbar.pack_start(&first_setup_window_headerbar_back_button);
    first_setup_window_toolbarview.add_top_bar(&first_setup_window_headerbar);

    first_setup_window_headerbar_back_button.connect_clicked(clone!(@weak first_setup_carousel => move |_| {
        let first_setup_prev_page = first_setup_carousel.position() - 1.0;
        first_setup_carousel.scroll_to(&first_setup_carousel.nth_page(first_setup_prev_page as u32), true)
    }));

    // CAROUSELS

    // Initial Carousel
    initial_carousel(&first_setup_carousel);
    internet_carousel(&first_setup_carousel, &internet_connected, &window);
    user_carousel(&first_setup_carousel);
    update_carousel(&first_setup_carousel, &internet_connected, &window);
    driver_carousel(&first_setup_carousel, &internet_connected);
    codec_carousel(&first_setup_carousel, &internet_connected, &window);
    gameutils_carousel(&first_setup_carousel, &internet_connected, &window);

    // Add file to window
    window.set_content(Some(&first_setup_window_toolbarview))
}
