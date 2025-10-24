use gtk4::glib::{self, ControlFlow};
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Button, Label, Orientation};
use ripclip_core::ipc::IpcStream;
use std::io::{Read, Write};

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id("ripclipui.gtk").build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(500)
            .title("Ripclip UI")
            .build();

        let vbox = GtkBox::new(Orientation::Vertical, 10);
        window.set_child(Some(&vbox));
        window.present();

        glib::timeout_add_local(std::time::Duration::from_secs(2), {
            let vbox = vbox.clone();
            move || {
                if let Ok(mut stream) = IpcStream::connect("/tmp/ripclip.sock") {
                    if stream.write_all(b"GET_COUNT").is_ok() {
                        let mut buf = [0; 4096];
                        if let Ok(len) = stream.read(&mut buf) {
                            let response = String::from_utf8_lossy(&buf[..len]);
                            println!("Respuesta: {}", response);

                            let items: Vec<String> = response
                                .split("Clip {")
                                .filter_map(|part| {
                                    if let Some(content) = part.split("content: ").nth(1) {
                                        let content = content.split(',').next()?.trim();
                                        Some(content.trim_matches('"').to_string())
                                    } else {
                                        None
                                    }
                                })
                                .collect();

                            while let Some(child) = vbox.first_child() {
                                vbox.remove(&child);
                            }

                            for item in items {
                                let row = GtkBox::new(Orientation::Horizontal, 5);
                                let label = Label::new(Some(&item));
                                let button = Button::with_label("Copiar");

                                button.set_hexpand(true);
                                button.set_halign(gtk4::Align::End);
                                button.set_margin_end(8);

                                let label_clone = label.clone();
                                button.connect_clicked(move |_| {
                                    println!("Copiado: {}", label_clone.text());
                                });

                                row.append(&label);
                                row.append(&button);
                                vbox.append(&row);
                            }
                        }
                    }
                }
                ControlFlow::Continue
            }
        });
    });

    app.run()
}
