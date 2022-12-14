use gtk::prelude::*;

mod gui;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.gtk_keyboard_events_listener"),
        Default::default(),
    );

    application.connect_activate(gui::build_ui);
    application.run();
}
