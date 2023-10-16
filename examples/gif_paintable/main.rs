// A custom GDK paintable capable of rendering a GIF
// The paintable makes uses of the awesome image
// crate to read a gif file and transform it to a Vec<Frame>
// which are then rendered by the paintable at different snapshots

mod gif_paintable;
mod gif_paintable_window;

use gif_paintable_window::GifPaintableWindow;
use gtk::{glib, prelude::*};

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.gif_paintable")
        .build();

    application.connect_activate(|app| {
        let win = GifPaintableWindow::new(app);
        win.present();
    });

    application.run()
}
