use gtk4::{
    gdk::Texture,
    gio::File,
    prelude::{ApplicationExt, ApplicationExtManual, BoxExt, GtkWindowExt, WidgetExt},
    Application, ApplicationWindow, Box, Button, HeaderBar, Image, Label,
};

fn main() {
    let app = Application::builder().application_id("mc.client").build();

    app.connect_activate(|app| {
        build_ui(app);
    });

    app.run();
}

fn build_ui(app: &Application) {
    let win = ApplicationWindow::builder()
        .application(app)
        .title("Main Window")
        .default_width(320)
        .default_height(200)
        .build();

    let header = HeaderBar::builder()
        .title_widget(&Label::new(Some("")))
        .build();

    let text_box = Box::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .spacing(10)
        .build();

    let instance = Label::new(Some("Test"));
    let version = Label::new(Some("1.21"));
    let loader = Label::new(Some("Forge/Fabric"));

    text_box.append(&instance);
    text_box.append(&version);
    text_box.append(&loader);

    header.pack_start(&text_box);

    let button = Button::builder().label("Play").build();
    header.pack_end(&button);

    win.set_titlebar(Some(&header));

    let container_box = Box::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .spacing(10)
        .build();

    let picture_box = Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .spacing(10)
        .build();

    let path = File::for_path("/image/here");
    let texture = Texture::from_file(&path).unwrap();

    // not working. need to fix
    //
    // supposed to return a loaded image from texture, fails to load.
    let picture = Image::builder().pixel_size(96).paintable(&texture).build();

    picture_box.append(&picture);

    let instance_label = Label::new(Some("Instance"));
    let instance_entry = Label::new(Some(
        "
Placeholder paragraph text for the instance. This
is a placeholder paragraph text for the instance.
Placeholder paragraph text for the instance.This is
a placeholder paragraph text for the instance.
",
    ));

    container_box.append(&picture_box);
    container_box.append(&instance_label);
    container_box.append(&instance_entry);

    win.set_child(Some(&container_box));

    win.show();
}
