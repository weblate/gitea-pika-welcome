// GTK crates
use adw::prelude::*;
use adw::*;
use glib::*;
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)



//
use std::cell::RefCell;
use std::rc::Rc;

// carousel crates
mod initial_carousel;
mod internet_carousel;
mod user_carousel;
mod update_carousel;
mod driver_carousel;
mod codec_carousel;
mod gameutils_carousel;
mod final_carousel;

use initial_carousel::*;
use internet_carousel::*;
use user_carousel::*;
use update_carousel::*;
use driver_carousel::*;
use codec_carousel::*;
use gameutils_carousel::*;
use final_carousel::*;

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

    let first_setup_window_headerbar_back_button = gtk::Button::builder().label(t!("first_setup_window_headerbar_back_button_label")).build();

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
    final_carousel(&first_setup_carousel);

    // Add file to window
    window.set_content(Some(&first_setup_window_toolbarview))
}
