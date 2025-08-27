mod check_envs;
mod asdcontrol_bind;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Orientation, Scale, Label, Box as GtkBox, Separator};
use check_envs::check_asdcontrol_command;
#[macro_use]
extern crate rust_i18n;

i18n!("locales", fallback = "en");

fn main() {
    let app = Application::builder()
        .application_id("com.sznowicki.asdcontrol-gnome")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("ASDControl GNOME - Multi Monitor")
            .default_width(600)
            .default_height(200)
            .build();

        let devices = check_envs::check_get_devices();
        
        if devices.is_empty() {
            // Show error if no devices found
            let error_label = Label::new(Some("No Apple Studio Display devices found"));
            window.set_child(Some(&error_label));
            window.show();
            return;
        }

        let main_container = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(15)
            .margin_top(20)
            .margin_bottom(20)
            .margin_start(20)
            .margin_end(20)
            .build();

        // Create controls for each device
        for (index, device) in devices.iter().enumerate() {
            let device_container = GtkBox::builder()
                .orientation(Orientation::Vertical)
                .spacing(8)
                .build();

            // Device label
            let device_label = Label::builder()
                .label(&format!("Apple Studio Display {} ({})", index + 1, device))
                .halign(gtk4::Align::Start)
                .build();
            device_label.add_css_class("heading");

            // Get current brightness value
            let bg_value = asdcontrol_bind::get_bg_value(device);

            // Create slider
            let slider = Scale::builder()
                .orientation(Orientation::Horizontal)
                .adjustment(&gtk4::Adjustment::new(bg_value as f64, 0.0, 100.0, 1.0, 5.0, 0.0))
                .hexpand(true)
                .draw_value(true)
                .value_pos(gtk4::PositionType::Right)
                .build();

            // Value label
            let value_label = Label::builder()
                .label(&format!("Brightness: {}%", bg_value))
                .halign(gtk4::Align::Start)
                .build();

            // Clone device string for the closure
            let device_clone = device.clone();
            let value_label_clone = value_label.clone();
            
            slider.connect_value_changed(move |s| {
                let value = s.value().round() as i32;
                asdcontrol_bind::set_bg_value(&device_clone, value);
                value_label_clone.set_label(&format!("Brightness: {}%", value));
                println!("Device {}: Set brightness to {}%", device_clone, value);
            });

            device_container.append(&device_label);
            device_container.append(&slider);
            device_container.append(&value_label);

            main_container.append(&device_container);

            // Add separator between devices (except for the last one)
            if index < devices.len() - 1 {
                let separator = Separator::builder()
                    .orientation(Orientation::Horizontal)
                    .margin_top(10)
                    .margin_bottom(5)
                    .build();
                main_container.append(&separator);
            }
        }

        window.set_child(Some(&main_container));
        window.show();
    });
    
    check_asdcontrol_command();
    app.run();
}
