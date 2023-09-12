mod consts;

use cursive::views::{Dialog, TextView};
use cursive::align::HAlign;
use cursive::view::Margins;

fn main() {
    let mut app = cursive::default();
    let logo = TextView::new(consts::LOGO);

    app.add_layer(Dialog::around(logo)
        .title("Pylon")
        .h_align(HAlign::Center)
        .padding(Margins::lrtb(5, 5, 2, 2))
        .button("Send", |s| s.quit())
        .button("Receive", |s| s.quit()));

    app.run();
}
