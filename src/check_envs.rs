use std::process::Command;
use gtk4::prelude::*;
use gtk4::{MessageDialog, MessageType, ButtonsType};
use std::path::PathBuf;

pub fn check_asdcontrol_command() {
    let asd_control = "asdcontrol";

    if Command::new(asd_control).output().is_err() {
        let app = gtk4::Application::builder()
            .application_id("com.sznowicki.asdcontrol-gnome-check")
            .build();

        app.connect_activate(|_| {
            show_error_modal(t!("error.noasdcontrol").as_ref());
        });

        app.run();
    }
}


pub fn check_get_devices() -> Vec<String> {
    let devices = find_all_devices();
    if devices.is_empty() {
        let app = gtk4::Application::builder()
            .application_id("com.sznowicki.asdcontrol-gnome-check")
            .build();

        app.connect_activate(move |_| {
            show_error_modal(t!("error.nodevice").as_ref());
        });

        app.run();
    }

    devices
}


pub fn check_get_device() -> Option<String> {
    find_all_devices().into_iter().next()
}

fn find_all_devices() -> Vec<String> {
    let mut paths = Vec::new();
    create_hiddev_paths(&mut paths, "/dev/usb".to_string());
    create_hiddev_paths(&mut paths, "/dev".to_string());

    let mut devices = Vec::new();

    // Check all paths that include "hiddev"
    for path in paths {
        if path.to_str().map_or(false, |p| p.contains("hiddev")) {
            let path_str = path.to_str().unwrap_or("");
            
            // Test if this device responds to asdcontrol
            if let Ok(output) = std::process::Command::new("asdcontrol")
                .arg("-s")
                .arg("-b")
                .arg(path_str)
                .output() 
            {
                let output_str = String::from_utf8_lossy(&output.stdout);
                // Check if the output contains a numeric value (brightness level)
                if output_str.trim().split_whitespace().any(|word| word.chars().all(|c| c.is_numeric())) {
                    devices.push(path_str.to_string());
                }
            }
        }
    }
    
    devices.sort();
    devices
}

fn create_hiddev_paths(paths: &mut Vec<PathBuf>, base: String) {
    if let Ok(entries) = std::fs::read_dir(base) {
        for entry in entries.filter_map(|entry| entry.ok()) {
            let path = entry.path();
            if !path.is_dir() && path.file_name().and_then(|name| name.to_str()).map_or(false, |name| name.starts_with("hiddev")) {
                paths.push(path);
            }
        }
    }
}

fn show_error_modal(message: &str) {
    let dialog = MessageDialog::builder()
        .message_type(MessageType::Error)
        .buttons(ButtonsType::Ok)
        .text(message)
        .modal(true)
        .build();
    dialog.connect_response(|dialog, _| dialog.close());
    dialog.show();
}
