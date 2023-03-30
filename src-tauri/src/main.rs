// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::{SystemTime, UNIX_EPOCH};

use chrono::prelude::Local;
use clipboard::{ClipboardContext, ClipboardProvider};
use rand::Rng;
use tauri::{
    ActivationPolicy, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, SystemTraySubmenu,
};
use uuid::Uuid;

fn set_clipboard(value: &str) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    ctx.set_contents(value.to_string()).unwrap();
}

fn main() {
    // System Tray settings
    let get_time_submenus = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("get_time_unix", "UNIX"))
        .add_item(CustomMenuItem::new("get_time_iso", "ISO"));
    let get_menus = SystemTrayMenu::new()
        .add_submenu(SystemTraySubmenu::new("Time", get_time_submenus));

    let generate_menus = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("generate_uuid".to_string(), "UUID v4"))
        .add_item(CustomMenuItem::new(
            "generate_mac".to_string(),
            "Mac Address",
        ))
        // TODO: Generate random ipv4 and ipv6 address
        .add_item(CustomMenuItem::new(
            "generate_random_32_string".to_string(),
            "Random 32 String",
        ));
    let quit = CustomMenuItem::new("quit".to_string(), "Quit").accelerator("Cmd+Q");
    let tray_menu = SystemTrayMenu::new()
        .add_submenu(SystemTraySubmenu::new("Get", get_menus))
        .add_submenu(SystemTraySubmenu::new("Generate", generate_menus))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .setup(|app| {
            app.set_activation_policy(ActivationPolicy::Accessory);
            Ok(())
        })
        .system_tray(system_tray)
        .on_system_tray_event(|_app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "get_time_unix" => {
                    let now = SystemTime::now();
                    let timestamp = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
                    set_clipboard(&timestamp.to_string());
                }
                "get_time_iso" => {
                    let dt = Local::now();
                    let iso_timestamp = dt.to_rfc3339();
                    set_clipboard(&iso_timestamp.to_string());
                }
                "generate_uuid" => {
                    let id = Uuid::new_v4();
                    set_clipboard(&id.to_string());
                }
                "generate_mac" => {
                    let mac_address: [u8; 6] = rand::thread_rng().gen();

                    // Print the MAC address in hex format
                    let mac = format!(
                        "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                        mac_address[0],
                        mac_address[1],
                        mac_address[2],
                        mac_address[3],
                        mac_address[4],
                        mac_address[5]
                    );
                    set_clipboard(&mac.to_string().to_lowercase());
                }
                "generate_random_32_string" => {
                    let hex_string: String = (0..16)
                        .map(|_| format!("{:02X}", rand::thread_rng().gen::<u8>()))
                        .collect();

                    set_clipboard(&hex_string.to_string().to_lowercase());
                }
                _ => {}
            },
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("Error while running toolkit")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
