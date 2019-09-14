extern crate cairo;
extern crate gtk;
extern crate gio;

use cairo::Context;
use gtk::prelude::*;
use gio::prelude::*;

use gtk::{Application, ApplicationWindow, DrawingArea};

fn on_draw(_widget: &DrawingArea, ctx: &Context) -> Inhibit {
    ctx.set_source_rgb(1.0, 0.0, 0.0);
    ctx.paint();
    Inhibit(false)
}

fn on_activate(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_title("Drawing Pixels");
    window.set_default_size(640, 480);
    let drawing_area = DrawingArea::new();
    drawing_area.connect_draw(on_draw);
    window.add(&drawing_area);
    window.show_all();
}

fn main() {
    let application = Application::new(Some("com.robjperez.gtkdrawing"), Default::default())
        .expect("failed to initialize GTK application");
    application.connect_activate(on_activate);
    application.run(&[]);
}

