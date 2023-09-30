mod consts;

use cursive::traits::*;
use cursive::views::{Dialog, LinearLayout, SelectView, TextView};
use cursive::align::HAlign;
use cursive::view::Margins;

fn main() {
    let mut app = cursive::default();
    let logo = TextView::new(consts::LOGO).h_align(HAlign::Center);

    let mut menu = SelectView::<&str>::new();
    let dashboard = Dialog::around(logo)
        .h_align(HAlign::Center)
        .padding(Margins::lrtb(5, 5, 2, 2))
        .button("Send", |s| s.quit())
        .button("Receive", |s| s.quit())
        .full_width();

    menu.add_item("Dashboard", "dashboard");
    menu.add_item("Active Transfers", "active_transfers");
    menu.add_item("History", "history");
    menu.add_item("Settings", "settings");
    menu.add_item("About", "settings");

    app.add_layer(Dialog::around(LinearLayout::horizontal()
        .child(Dialog::around(menu))
        .child(dashboard))
        .title("Pylon")
        .full_screen());

    app.run();
}
