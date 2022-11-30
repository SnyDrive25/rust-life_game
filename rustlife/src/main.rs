use fltk::{app, prelude::*, window::Window};

fn main() {
    let app = app::App::default();
    let mut w = Window::new(100, 100, 400, 300, "Jeu de la vie");
    w.end();
    w.show();
    app.run().unwrap();
}
