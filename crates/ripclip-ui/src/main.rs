use gtk4::glib::{self, clone};
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Button, Label, Orientation};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("ripclipui.gtk")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(500)
            .title("Ripclip UI")
            .build();

        let vbox = GtkBox::new(Orientation::Vertical, 10);

        let items = vec!["Clip 1", "Clip 2", "Clip 3"];

        for item in items {
            let row = GtkBox::new(Orientation::Horizontal, 5);

            let label = Label::new(Some(item));

            let button = Button::with_label("Copiar");
            let label_clone = label.clone();
            button.connect_clicked(move |_| {
                println!("Copiado: {}", label_clone.text());
            });

            row.append(&label);
            row.append(&button);

            vbox.append(&row);
        }

        window.set_child(Some(&vbox));
        window.present();
    });

    app.run()
}
