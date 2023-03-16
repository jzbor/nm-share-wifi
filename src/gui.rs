use gtk4::prelude::*;
use gtk4::{
    Align,
    Application,
    ApplicationWindow,
    Box,
    ComboBoxText,
    Image,
    Label,
    Orientation,
    PasswordEntry,
    glib,
};

use crate::network::WifiNetwork;


const APP_ID: &str = "de.jzbor.nm-share-wifi";
const TITLE: &str = "NetworManager Share Wifi";


struct Widgets {
    qr_image: Image,
    ssid_label: Label,
    pass_entry: PasswordEntry,
}


pub fn run_gui(args: Vec<String>) -> Result<(), String> {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);

    let exit_code = app.run_with_args(&args);

    if exit_code == glib::ExitCode::FAILURE {
        return Err(format!("GTK-Application returned with error"));
    }

    Ok(())
}

fn build_ui(app: &Application) {
    let wifis = WifiNetwork::nm_wifis().unwrap();

    // Create the wifi selector
    let wifi_selector = ComboBoxText::builder()
        .build();
    for (name, _) in &wifis {
        wifi_selector.append_text(&name);
    }

    // QR code
    let image_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .homogeneous(true)
        .spacing(5)
        .halign(Align::Center)
        .build();
    let qr_image = Image::builder()
        .build();
    image_box.append(&qr_image);

    // Create the SSID label
    let ssid_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .homogeneous(true)
        .spacing(5)
        .halign(Align::Center)
        .build();
    let ssid_label = Label::builder()
        .label("Wifi Network")
        .build();
    ssid_box.append(&ssid_label);

    let pass_entry = PasswordEntry::builder()
        .editable(false)
        .can_focus(false)
        .show_peek_icon(true)
        .placeholder_text("Wifi Password")
        .build();
    let passkey_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .homogeneous(true)
        .spacing(5)
        .halign(Align::Center)
        .build();
    passkey_box.append(&pass_entry);

    let main_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(20)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();
    main_box.append(&wifi_selector);
    main_box.append(&image_box);
    main_box.append(&ssid_box);
    main_box.append(&passkey_box);


    let widgets = Widgets { qr_image, ssid_label, pass_entry };
    wifi_selector.connect_changed(move |selector| wifi_selection_changed(selector, &widgets, &wifis));

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title(TITLE)
        .default_width(650)
        .default_height(850)
        .child(&main_box)
        .build();

    // Present window
    window.present();
}

fn wifi_selection_changed(selector: &ComboBoxText, widgets: &Widgets, wifis: &[(String, WifiNetwork)]) {
    if let Some(text) = selector.active_text() {
        let text = text.to_string();
        if let Some(wifi) = wifis.iter().find(|(n, _)| n == &text).map(|(_, w)| w) {
            widgets.ssid_label.set_label(wifi.ssid());
            widgets.pass_entry.set_text(wifi.passkey());
            if let Ok(qr_code) = wifi.qr_code() {
                let pixbuf = qr_code.to_gdk_pixbuf();
                widgets.qr_image.set_pixel_size(pixbuf.width());
                widgets.qr_image.set_from_pixbuf(Some(&pixbuf));
            }
        }
    }
}
